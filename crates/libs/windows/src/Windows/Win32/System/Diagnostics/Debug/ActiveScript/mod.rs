pub const ACTIVPROF_E_PROFILER_ABSENT: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const ACTIVPROF_E_PROFILER_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const ACTIVPROF_E_UNABLE_TO_APPLY_ACTION: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const APPBREAKFLAG_DEBUGGER_BLOCK: u32 = 1u32;
pub const APPBREAKFLAG_DEBUGGER_HALT: u32 = 2u32;
pub const APPBREAKFLAG_IN_BREAKPOINT: u32 = 2147483648u32;
pub const APPBREAKFLAG_NESTED: u32 = 131072u32;
pub const APPBREAKFLAG_STEP: u32 = 65536u32;
pub const APPBREAKFLAG_STEPTYPE_BYTECODE: u32 = 1048576u32;
pub const APPBREAKFLAG_STEPTYPE_MACHINE: u32 = 2097152u32;
pub const APPBREAKFLAG_STEPTYPE_MASK: u32 = 15728640u32;
pub const APPBREAKFLAG_STEPTYPE_SOURCE: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPLICATION_NODE_EVENT_FILTER(pub i32);
windows_core::imp::define_interface!(AsyncIDebugApplicationNodeEvents, AsyncIDebugApplicationNodeEvents_Vtbl, 0xa2e3aa3b_aa8d_4ebf_84cd_648b737b8c13);
windows_core::imp::interface_hierarchy!(AsyncIDebugApplicationNodeEvents, windows_core::IUnknown);
impl AsyncIDebugApplicationNodeEvents {
    pub unsafe fn Begin_onAddChild<P0>(&self, prddpchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_onAddChild)(windows_core::Interface::as_raw(self), prddpchild.param().abi()).ok() }
    }
    pub unsafe fn Finish_onAddChild(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_onAddChild)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_onRemoveChild<P0>(&self, prddpchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_onRemoveChild)(windows_core::Interface::as_raw(self), prddpchild.param().abi()).ok() }
    }
    pub unsafe fn Finish_onRemoveChild(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_onRemoveChild)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_onDetach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Begin_onDetach)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finish_onDetach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_onDetach)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Begin_onAttach<P0>(&self, prddpparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).Begin_onAttach)(windows_core::Interface::as_raw(self), prddpparent.param().abi()).ok() }
    }
    pub unsafe fn Finish_onAttach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finish_onAttach)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIDebugApplicationNodeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_onAddChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_onAddChild: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_onRemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_onRemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_onDetach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_onDetach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_onAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_onAttach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIDebugApplicationNodeEvents_Impl: windows_core::IUnknownImpl {
    fn Begin_onAddChild(&self, prddpchild: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn Finish_onAddChild(&self) -> windows_core::Result<()>;
    fn Begin_onRemoveChild(&self, prddpchild: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn Finish_onRemoveChild(&self) -> windows_core::Result<()>;
    fn Begin_onDetach(&self) -> windows_core::Result<()>;
    fn Finish_onDetach(&self) -> windows_core::Result<()>;
    fn Begin_onAttach(&self, prddpparent: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn Finish_onAttach(&self) -> windows_core::Result<()>;
}
impl AsyncIDebugApplicationNodeEvents_Vtbl {
    pub const fn new<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_onAddChild<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Begin_onAddChild(this, core::mem::transmute_copy(&prddpchild)).into()
            }
        }
        unsafe extern "system" fn Finish_onAddChild<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Finish_onAddChild(this).into()
            }
        }
        unsafe extern "system" fn Begin_onRemoveChild<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Begin_onRemoveChild(this, core::mem::transmute_copy(&prddpchild)).into()
            }
        }
        unsafe extern "system" fn Finish_onRemoveChild<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Finish_onRemoveChild(this).into()
            }
        }
        unsafe extern "system" fn Begin_onDetach<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Begin_onDetach(this).into()
            }
        }
        unsafe extern "system" fn Finish_onDetach<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Finish_onDetach(this).into()
            }
        }
        unsafe extern "system" fn Begin_onAttach<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpparent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Begin_onAttach(this, core::mem::transmute_copy(&prddpparent)).into()
            }
        }
        unsafe extern "system" fn Finish_onAttach<Identity: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIDebugApplicationNodeEvents_Impl::Finish_onAttach(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_onAddChild: Begin_onAddChild::<Identity, OFFSET>,
            Finish_onAddChild: Finish_onAddChild::<Identity, OFFSET>,
            Begin_onRemoveChild: Begin_onRemoveChild::<Identity, OFFSET>,
            Finish_onRemoveChild: Finish_onRemoveChild::<Identity, OFFSET>,
            Begin_onDetach: Begin_onDetach::<Identity, OFFSET>,
            Finish_onDetach: Finish_onDetach::<Identity, OFFSET>,
            Begin_onAttach: Begin_onAttach::<Identity, OFFSET>,
            Finish_onAttach: Finish_onAttach::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIDebugApplicationNodeEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIDebugApplicationNodeEvents {}
pub const BREAKPOINT_DELETED: BREAKPOINT_STATE = BREAKPOINT_STATE(0i32);
pub const BREAKPOINT_DISABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(1i32);
pub const BREAKPOINT_ENABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BREAKPOINT_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BREAKREASON(pub i32);
pub const BREAKREASON_BREAKPOINT: BREAKREASON = BREAKREASON(1i32);
pub const BREAKREASON_DEBUGGER_BLOCK: BREAKREASON = BREAKREASON(2i32);
pub const BREAKREASON_DEBUGGER_HALT: BREAKREASON = BREAKREASON(5i32);
pub const BREAKREASON_ERROR: BREAKREASON = BREAKREASON(6i32);
pub const BREAKREASON_HOST_INITIATED: BREAKREASON = BREAKREASON(3i32);
pub const BREAKREASON_JIT: BREAKREASON = BREAKREASON(7i32);
pub const BREAKREASON_LANGUAGE_INITIATED: BREAKREASON = BREAKREASON(4i32);
pub const BREAKREASON_MUTATION_BREAKPOINT: BREAKREASON = BREAKREASON(8i32);
pub const BREAKREASON_STEP: BREAKREASON = BREAKREASON(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BREAKRESUMEACTION(pub i32);
pub const BREAKRESUMEACTION_ABORT: BREAKRESUMEACTION = BREAKRESUMEACTION(0i32);
pub const BREAKRESUMEACTION_CONTINUE: BREAKRESUMEACTION = BREAKRESUMEACTION(1i32);
pub const BREAKRESUMEACTION_IGNORE: BREAKRESUMEACTION = BREAKRESUMEACTION(5i32);
pub const BREAKRESUMEACTION_STEP_DOCUMENT: BREAKRESUMEACTION = BREAKRESUMEACTION(6i32);
pub const BREAKRESUMEACTION_STEP_INTO: BREAKRESUMEACTION = BREAKRESUMEACTION(2i32);
pub const BREAKRESUMEACTION_STEP_OUT: BREAKRESUMEACTION = BREAKRESUMEACTION(4i32);
pub const BREAKRESUMEACTION_STEP_OVER: BREAKRESUMEACTION = BREAKRESUMEACTION(3i32);
pub const CATID_ActiveScript: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a1_9847_11cf_8f20_00805f2cd064);
pub const CATID_ActiveScriptAuthor: windows_core::GUID = windows_core::GUID::from_u128(0x0aee2a92_bcbb_11d0_8c72_00c04fc2b085);
pub const CATID_ActiveScriptEncode: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a3_9847_11cf_8f20_00805f2cd064);
pub const CATID_ActiveScriptParse: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a2_9847_11cf_8f20_00805f2cd064);
pub const CDebugDocumentHelper: windows_core::GUID = windows_core::GUID::from_u128(0x83b8bca6_687c_11d0_a405_00aa0060275c);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEBUG_EVENT_INFO_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEBUG_STACKFRAME_TYPE(pub i32);
pub const DEBUG_TEXT_ALLOWBREAKPOINTS: u32 = 8u32;
pub const DEBUG_TEXT_ALLOWERRORREPORT: u32 = 16u32;
pub const DEBUG_TEXT_EVALUATETOCODECONTEXT: u32 = 32u32;
pub const DEBUG_TEXT_ISEXPRESSION: u32 = 1u32;
pub const DEBUG_TEXT_ISNONUSERCODE: u32 = 64u32;
pub const DEBUG_TEXT_NOSIDEEFFECTS: u32 = 4u32;
pub const DEBUG_TEXT_RETURNVALUE: u32 = 2u32;
pub const DEIT_ASMJS_FAILED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(3i32);
pub const DEIT_ASMJS_IN_DEBUGGING: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(1i32);
pub const DEIT_ASMJS_SUCCEEDED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(2i32);
pub const DEIT_GENERAL: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DOCUMENTNAMETYPE(pub i32);
pub const DOCUMENTNAMETYPE_APPNODE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(0i32);
pub const DOCUMENTNAMETYPE_FILE_TAIL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(2i32);
pub const DOCUMENTNAMETYPE_SOURCE_MAP_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(5i32);
pub const DOCUMENTNAMETYPE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(1i32);
pub const DOCUMENTNAMETYPE_UNIQUE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(4i32);
pub const DOCUMENTNAMETYPE_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(3i32);
pub const DST_INTERNAL_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(1i32);
pub const DST_INVOCATION_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(2i32);
pub const DST_SCRIPT_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(0i32);
pub const DebugHelper: windows_core::GUID = windows_core::GUID::from_u128(0x0bfcc060_8c1d_11d0_accd_00aa0060275c);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugStackFrameDescriptor {
    pub pdsf: core::mem::ManuallyDrop<Option<IDebugStackFrame>>,
    pub dwMin: u32,
    pub dwLim: u32,
    pub fFinal: windows_core::BOOL,
    pub punkFinal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugStackFrameDescriptor64 {
    pub pdsf: core::mem::ManuallyDrop<Option<IDebugStackFrame>>,
    pub dwMin: u64,
    pub dwLim: u64,
    pub fFinal: windows_core::BOOL,
    pub punkFinal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
pub const DefaultDebugSessionProvider: windows_core::GUID = windows_core::GUID::from_u128(0x834128a2_51f4_11d0_8f20_00805f2cd064);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ERRORRESUMEACTION(pub i32);
pub const ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller: ERRORRESUMEACTION = ERRORRESUMEACTION(1i32);
pub const ERRORRESUMEACTION_ReexecuteErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(0i32);
pub const ERRORRESUMEACTION_SkipErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(2i32);
pub const ETK_FIRST_CHANCE: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(0i32);
pub const ETK_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(2i32);
pub const ETK_USER_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(1i32);
pub const E_JsDEBUG_INVALID_MEMORY_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x8DC70005_u32 as _);
pub const E_JsDEBUG_MISMATCHED_RUNTIME: windows_core::HRESULT = windows_core::HRESULT(0x8DC70001_u32 as _);
pub const E_JsDEBUG_OUTSIDE_OF_VM: windows_core::HRESULT = windows_core::HRESULT(0x8DC70004_u32 as _);
pub const E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE: windows_core::HRESULT = windows_core::HRESULT(0x8DC70007_u32 as _);
pub const E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8DC70006_u32 as _);
pub const E_JsDEBUG_UNKNOWN_THREAD: windows_core::HRESULT = windows_core::HRESULT(0x8DC70002_u32 as _);
pub const FACILITY_JsDEBUG: u32 = 3527u32;
pub const FILTER_EXCLUDE_ANONYMOUS_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(1i32);
pub const FILTER_EXCLUDE_EVAL_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(2i32);
pub const FILTER_EXCLUDE_NOTHING: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(0i32);
pub const GETATTRFLAG_HUMANTEXT: u32 = 32768u32;
pub const GETATTRFLAG_THIS: u32 = 256u32;
pub const GETATTRTYPE_DEPSCAN: u32 = 1u32;
pub const GETATTRTYPE_NORMAL: u32 = 0u32;
windows_core::imp::define_interface!(IActiveScript, IActiveScript_Vtbl, 0xbb1a2ae1_a4f9_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScript, windows_core::IUnknown);
impl IActiveScript {
    pub unsafe fn SetScriptSite<P0>(&self, pass: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScriptSite)(windows_core::Interface::as_raw(self), pass.param().abi()).ok() }
    }
    pub unsafe fn GetScriptSite<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetScriptSite)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn SetScriptState(&self, ss: SCRIPTSTATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScriptState)(windows_core::Interface::as_raw(self), ss).ok() }
    }
    pub unsafe fn GetScriptState(&self) -> windows_core::Result<SCRIPTSTATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddNamedItem<P0>(&self, pstrname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNamedItem)(windows_core::Interface::as_raw(self), pstrname.param().abi(), dwflags).ok() }
    }
    pub unsafe fn AddTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTypeLib)(windows_core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor, dwflags).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetScriptDispatch<P0>(&self, pstritemname: P0) -> windows_core::Result<super::super::super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptDispatch)(windows_core::Interface::as_raw(self), pstritemname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentScriptThreadID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentScriptThreadID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScriptThreadID(&self, dwwin32threadid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptThreadID)(windows_core::Interface::as_raw(self), dwwin32threadid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScriptThreadState(&self, stidthread: u32) -> windows_core::Result<SCRIPTTHREADSTATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptThreadState)(windows_core::Interface::as_raw(self), stidthread, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InterruptScriptThread(&self, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InterruptScriptThread)(windows_core::Interface::as_raw(self), stidthread, core::mem::transmute(pexcepinfo), dwflags).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IActiveScript> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScript_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetScriptSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScriptSite: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScriptState: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPTSTATE) -> windows_core::HRESULT,
    pub GetScriptState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCRIPTSTATE) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetScriptDispatch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetScriptDispatch: usize,
    pub GetCurrentScriptThreadID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScriptThreadID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetScriptThreadState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut SCRIPTTHREADSTATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InterruptScriptThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Com::EXCEPINFO, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InterruptScriptThread: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScript_Impl: windows_core::IUnknownImpl {
    fn SetScriptSite(&self, pass: windows_core::Ref<'_, IActiveScriptSite>) -> windows_core::Result<()>;
    fn GetScriptSite(&self, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetScriptState(&self, ss: SCRIPTSTATE) -> windows_core::Result<()>;
    fn GetScriptState(&self) -> windows_core::Result<SCRIPTSTATE>;
    fn Close(&self) -> windows_core::Result<()>;
    fn AddNamedItem(&self, pstrname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn AddTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetScriptDispatch(&self, pstritemname: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Com::IDispatch>;
    fn GetCurrentScriptThreadID(&self) -> windows_core::Result<u32>;
    fn GetScriptThreadID(&self, dwwin32threadid: u32) -> windows_core::Result<u32>;
    fn GetScriptThreadState(&self, stidthread: u32) -> windows_core::Result<SCRIPTTHREADSTATE>;
    fn InterruptScriptThread(&self, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IActiveScript>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScript_Vtbl {
    pub const fn new<Identity: IActiveScript_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetScriptSite<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pass: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::SetScriptSite(this, core::mem::transmute_copy(&pass)).into()
            }
        }
        unsafe extern "system" fn GetScriptSite<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::GetScriptSite(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn SetScriptState<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ss: SCRIPTSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::SetScriptState(this, core::mem::transmute_copy(&ss)).into()
            }
        }
        unsafe extern "system" fn GetScriptState<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pssstate: *mut SCRIPTSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::GetScriptState(this) {
                    Ok(ok__) => {
                        pssstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn AddNamedItem<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::AddNamedItem(this, core::mem::transmute(&pstrname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn AddTypeLib<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::AddTypeLib(this, core::mem::transmute_copy(&rguidtypelib), core::mem::transmute_copy(&dwmajor), core::mem::transmute_copy(&dwminor), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetScriptDispatch<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstritemname: windows_core::PCWSTR, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::GetScriptDispatch(this, core::mem::transmute(&pstritemname)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentScriptThreadID<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstidthread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::GetCurrentScriptThreadID(this) {
                    Ok(ok__) => {
                        pstidthread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptThreadID<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwin32threadid: u32, pstidthread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::GetScriptThreadID(this, core::mem::transmute_copy(&dwwin32threadid)) {
                    Ok(ok__) => {
                        pstidthread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptThreadState<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stidthread: u32, pstsstate: *mut SCRIPTTHREADSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::GetScriptThreadState(this, core::mem::transmute_copy(&stidthread)) {
                    Ok(ok__) => {
                        pstsstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InterruptScriptThread<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScript_Impl::InterruptScriptThread(this, core::mem::transmute_copy(&stidthread), core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IActiveScript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscript: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScript_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppscript.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScriptSite: SetScriptSite::<Identity, OFFSET>,
            GetScriptSite: GetScriptSite::<Identity, OFFSET>,
            SetScriptState: SetScriptState::<Identity, OFFSET>,
            GetScriptState: GetScriptState::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            AddNamedItem: AddNamedItem::<Identity, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, OFFSET>,
            GetScriptDispatch: GetScriptDispatch::<Identity, OFFSET>,
            GetCurrentScriptThreadID: GetCurrentScriptThreadID::<Identity, OFFSET>,
            GetScriptThreadID: GetScriptThreadID::<Identity, OFFSET>,
            GetScriptThreadState: GetScriptThreadState::<Identity, OFFSET>,
            InterruptScriptThread: InterruptScriptThread::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScript as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScript {}
windows_core::imp::define_interface!(IActiveScriptAuthor, IActiveScriptAuthor_Vtbl, 0x9c109da0_7006_11d1_b36c_00a0c911e8b2);
windows_core::imp::interface_hierarchy!(IActiveScriptAuthor, windows_core::IUnknown);
impl IActiveScriptAuthor {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddNamedItem<P0, P2>(&self, pszname: P0, dwflags: u32, pdisp: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNamedItem)(windows_core::Interface::as_raw(self), pszname.param().abi(), dwflags, pdisp.param().abi()).ok() }
    }
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pszdefaultname: P0, pszcode: P1, pszitemname: P2, pszsubitemname: P3, pszeventname: P4, pszdelimiter: P5, dwcookie: u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddScriptlet)(windows_core::Interface::as_raw(self), pszdefaultname.param().abi(), pszcode.param().abi(), pszitemname.param().abi(), pszsubitemname.param().abi(), pszeventname.param().abi(), pszdelimiter.param().abi(), dwcookie, dwflags).ok() }
    }
    pub unsafe fn ParseScriptText<P0, P1, P2>(&self, pszcode: P0, pszitemname: P1, pszdelimiter: P2, dwcookie: u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseScriptText)(windows_core::Interface::as_raw(self), pszcode.param().abi(), pszitemname.param().abi(), pszdelimiter.param().abi(), dwcookie, dwflags).ok() }
    }
    pub unsafe fn GetScriptTextAttributes<P0, P2>(&self, pszcode: P0, cch: u32, pszdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptTextAttributes)(windows_core::Interface::as_raw(self), pszcode.param().abi(), cch, pszdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P2>(&self, pszcode: P0, cch: u32, pszdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptletTextAttributes)(windows_core::Interface::as_raw(self), pszcode.param().abi(), cch, pszdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn GetRoot(&self) -> windows_core::Result<IScriptNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLanguageFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEventHandler<P0, P1, P2, P3>(&self, pdisp: P0, pszitem: P1, pszsubitem: P2, pszevent: P3) -> windows_core::Result<IScriptEntry>
    where
        P0: windows_core::Param<super::super::super::Com::IDispatch>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventHandler)(windows_core::Interface::as_raw(self), pdisp.param().abi(), pszitem.param().abi(), pszsubitem.param().abi(), pszevent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveNamedItem<P0>(&self, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveNamedItem)(windows_core::Interface::as_raw(self), pszname.param().abi()).ok() }
    }
    pub unsafe fn AddTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddTypeLib)(windows_core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor, dwflags).ok() }
    }
    pub unsafe fn RemoveTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveTypeLib)(windows_core::Interface::as_raw(self), rguidtypelib, dwmajor, dwminor).ok() }
    }
    pub unsafe fn GetChars(&self, frequestedlist: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChars)(windows_core::Interface::as_raw(self), frequestedlist, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetInfoFromContext<P0>(&self, pszcode: P0, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetInfoFromContext)(windows_core::Interface::as_raw(self), pszcode.param().abi(), cchcode, ichcurrentposition, dwlisttypesrequested, pdwlisttypesprovided as _, pichlistanchorposition as _, pichfuncanchorposition as _, pmemid as _, picurrentparameter as _, core::mem::transmute(ppunk)).ok() }
    }
    pub unsafe fn IsCommitChar(&self, ch: u16) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCommitChar)(windows_core::Interface::as_raw(self), ch, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptAuthor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddNamedItem: usize,
    pub AddScriptlet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub ParseScriptText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32) -> windows_core::HRESULT,
    pub GetScriptTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLanguageFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEventHandler: usize,
    pub RemoveNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, u32) -> windows_core::HRESULT,
    pub RemoveTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub GetChars: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInfoFromContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut u32, *mut u32, *mut u32, *mut i32, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCommitChar: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptAuthor_Impl: windows_core::IUnknownImpl {
    fn AddNamedItem(&self, pszname: &windows_core::PCWSTR, dwflags: u32, pdisp: windows_core::Ref<'_, super::super::super::Com::IDispatch>) -> windows_core::Result<()>;
    fn AddScriptlet(&self, pszdefaultname: &windows_core::PCWSTR, pszcode: &windows_core::PCWSTR, pszitemname: &windows_core::PCWSTR, pszsubitemname: &windows_core::PCWSTR, pszeventname: &windows_core::PCWSTR, pszdelimiter: &windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> windows_core::Result<()>;
    fn ParseScriptText(&self, pszcode: &windows_core::PCWSTR, pszitemname: &windows_core::PCWSTR, pszdelimiter: &windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> windows_core::Result<()>;
    fn GetScriptTextAttributes(&self, pszcode: &windows_core::PCWSTR, cch: u32, pszdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pszcode: &windows_core::PCWSTR, cch: u32, pszdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn GetRoot(&self) -> windows_core::Result<IScriptNode>;
    fn GetLanguageFlags(&self) -> windows_core::Result<u32>;
    fn GetEventHandler(&self, pdisp: windows_core::Ref<'_, super::super::super::Com::IDispatch>, pszitem: &windows_core::PCWSTR, pszsubitem: &windows_core::PCWSTR, pszevent: &windows_core::PCWSTR) -> windows_core::Result<IScriptEntry>;
    fn RemoveNamedItem(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::Result<()>;
    fn RemoveTypeLib(&self, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32) -> windows_core::Result<()>;
    fn GetChars(&self, frequestedlist: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetInfoFromContext(&self, pszcode: &windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: windows_core::OutRef<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn IsCommitChar(&self, ch: u16) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptAuthor_Vtbl {
    pub const fn new<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddNamedItem<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, dwflags: u32, pdisp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::AddNamedItem(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdisp)).into()
            }
        }
        unsafe extern "system" fn AddScriptlet<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultname: windows_core::PCWSTR, pszcode: windows_core::PCWSTR, pszitemname: windows_core::PCWSTR, pszsubitemname: windows_core::PCWSTR, pszeventname: windows_core::PCWSTR, pszdelimiter: windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::AddScriptlet(this, core::mem::transmute(&pszdefaultname), core::mem::transmute(&pszcode), core::mem::transmute(&pszitemname), core::mem::transmute(&pszsubitemname), core::mem::transmute(&pszeventname), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ParseScriptText<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcode: windows_core::PCWSTR, pszitemname: windows_core::PCWSTR, pszdelimiter: windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::ParseScriptText(this, core::mem::transmute(&pszcode), core::mem::transmute(&pszitemname), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcode: windows_core::PCWSTR, cch: u32, pszdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::GetScriptTextAttributes(this, core::mem::transmute(&pszcode), core::mem::transmute_copy(&cch), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcode: windows_core::PCWSTR, cch: u32, pszdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::GetScriptletTextAttributes(this, core::mem::transmute(&pszcode), core::mem::transmute_copy(&cch), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn GetRoot<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptAuthor_Impl::GetRoot(this) {
                    Ok(ok__) => {
                        ppsp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguageFlags<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrfasa: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptAuthor_Impl::GetLanguageFlags(this) {
                    Ok(ok__) => {
                        pgrfasa.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventHandler<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisp: *mut core::ffi::c_void, pszitem: windows_core::PCWSTR, pszsubitem: windows_core::PCWSTR, pszevent: windows_core::PCWSTR, ppse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptAuthor_Impl::GetEventHandler(this, core::mem::transmute_copy(&pdisp), core::mem::transmute(&pszitem), core::mem::transmute(&pszsubitem), core::mem::transmute(&pszevent)) {
                    Ok(ok__) => {
                        ppse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveNamedItem<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::RemoveNamedItem(this, core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn AddTypeLib<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::AddTypeLib(this, core::mem::transmute_copy(&rguidtypelib), core::mem::transmute_copy(&dwmajor), core::mem::transmute_copy(&dwminor), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn RemoveTypeLib<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidtypelib: *const windows_core::GUID, dwmajor: u32, dwminor: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::RemoveTypeLib(this, core::mem::transmute_copy(&rguidtypelib), core::mem::transmute_copy(&dwmajor), core::mem::transmute_copy(&dwminor)).into()
            }
        }
        unsafe extern "system" fn GetChars<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequestedlist: u32, pbstrchars: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptAuthor_Impl::GetChars(this, core::mem::transmute_copy(&frequestedlist)) {
                    Ok(ok__) => {
                        pbstrchars.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfoFromContext<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcode: windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthor_Impl::GetInfoFromContext(this, core::mem::transmute(&pszcode), core::mem::transmute_copy(&cchcode), core::mem::transmute_copy(&ichcurrentposition), core::mem::transmute_copy(&dwlisttypesrequested), core::mem::transmute_copy(&pdwlisttypesprovided), core::mem::transmute_copy(&pichlistanchorposition), core::mem::transmute_copy(&pichfuncanchorposition), core::mem::transmute_copy(&pmemid), core::mem::transmute_copy(&picurrentparameter), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn IsCommitChar<Identity: IActiveScriptAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ch: u16, pfcommit: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptAuthor_Impl::IsCommitChar(this, core::mem::transmute_copy(&ch)) {
                    Ok(ok__) => {
                        pfcommit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamedItem: AddNamedItem::<Identity, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, OFFSET>,
            GetRoot: GetRoot::<Identity, OFFSET>,
            GetLanguageFlags: GetLanguageFlags::<Identity, OFFSET>,
            GetEventHandler: GetEventHandler::<Identity, OFFSET>,
            RemoveNamedItem: RemoveNamedItem::<Identity, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, OFFSET>,
            RemoveTypeLib: RemoveTypeLib::<Identity, OFFSET>,
            GetChars: GetChars::<Identity, OFFSET>,
            GetInfoFromContext: GetInfoFromContext::<Identity, OFFSET>,
            IsCommitChar: IsCommitChar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptAuthor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptAuthor {}
windows_core::imp::define_interface!(IActiveScriptAuthorProcedure, IActiveScriptAuthorProcedure_Vtbl, 0x7e2d4b70_bd9a_11d0_9336_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IActiveScriptAuthorProcedure, windows_core::IUnknown);
impl IActiveScriptAuthorProcedure {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P7>(&self, pszcode: P0, pszformalparams: P1, pszprocedurename: P2, pszitemname: P3, pszdelimiter: P4, dwcookie: u32, dwflags: u32, pdispfor: P7) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<super::super::super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseProcedureText)(windows_core::Interface::as_raw(self), pszcode.param().abi(), pszformalparams.param().abi(), pszprocedurename.param().abi(), pszitemname.param().abi(), pszdelimiter.param().abi(), dwcookie, dwflags, pdispfor.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptAuthorProcedure_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptAuthorProcedure_Impl: windows_core::IUnknownImpl {
    fn ParseProcedureText(&self, pszcode: &windows_core::PCWSTR, pszformalparams: &windows_core::PCWSTR, pszprocedurename: &windows_core::PCWSTR, pszitemname: &windows_core::PCWSTR, pszdelimiter: &windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: windows_core::Ref<'_, super::super::super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptAuthorProcedure_Vtbl {
    pub const fn new<Identity: IActiveScriptAuthorProcedure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseProcedureText<Identity: IActiveScriptAuthorProcedure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcode: windows_core::PCWSTR, pszformalparams: windows_core::PCWSTR, pszprocedurename: windows_core::PCWSTR, pszitemname: windows_core::PCWSTR, pszdelimiter: windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptAuthorProcedure_Impl::ParseProcedureText(this, core::mem::transmute(&pszcode), core::mem::transmute(&pszformalparams), core::mem::transmute(&pszprocedurename), core::mem::transmute(&pszitemname), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdispfor)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptAuthorProcedure as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptAuthorProcedure {}
windows_core::imp::define_interface!(IActiveScriptDebug32, IActiveScriptDebug32_Vtbl, 0x51973c10_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IActiveScriptDebug32, windows_core::IUnknown);
impl IActiveScriptDebug32 {
    pub unsafe fn GetScriptTextAttributes<P0, P2>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptTextAttributes)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), unumcodechars, pstrdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P2>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptletTextAttributes)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), unumcodechars, pstrdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IEnumDebugCodeContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodeContextsOfPosition)(windows_core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptDebug32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetScriptTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub EnumCodeContextsOfPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptDebug32_Impl: windows_core::IUnknownImpl {
    fn GetScriptTextAttributes(&self, pstrcode: &windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pstrcode: &windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IEnumDebugCodeContexts>;
}
impl IActiveScriptDebug32_Vtbl {
    pub const fn new<Identity: IActiveScriptDebug32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptDebug32_Impl::GetScriptTextAttributes(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&unumcodechars), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptDebug32_Impl::GetScriptletTextAttributes(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&unumcodechars), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: IActiveScriptDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptDebug32_Impl::EnumCodeContextsOfPosition(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ucharacteroffset), core::mem::transmute_copy(&unumchars)) {
                    Ok(ok__) => {
                        ppescc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptDebug32 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptDebug32 {}
windows_core::imp::define_interface!(IActiveScriptDebug64, IActiveScriptDebug64_Vtbl, 0xbc437e23_f5b8_47f4_bb79_7d1ce5483b86);
windows_core::imp::interface_hierarchy!(IActiveScriptDebug64, windows_core::IUnknown);
impl IActiveScriptDebug64 {
    pub unsafe fn GetScriptTextAttributes<P0, P2>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptTextAttributes)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), unumcodechars, pstrdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn GetScriptletTextAttributes<P0, P2>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptletTextAttributes)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), unumcodechars, pstrdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IEnumDebugCodeContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodeContextsOfPosition)(windows_core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptDebug64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetScriptTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub GetScriptletTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub EnumCodeContextsOfPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptDebug64_Impl: windows_core::IUnknownImpl {
    fn GetScriptTextAttributes(&self, pstrcode: &windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn GetScriptletTextAttributes(&self, pstrcode: &windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IEnumDebugCodeContexts>;
}
impl IActiveScriptDebug64_Vtbl {
    pub const fn new<Identity: IActiveScriptDebug64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptDebug64_Impl::GetScriptTextAttributes(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&unumcodechars), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptDebug64_Impl::GetScriptletTextAttributes(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&unumcodechars), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: IActiveScriptDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptDebug64_Impl::EnumCodeContextsOfPosition(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ucharacteroffset), core::mem::transmute_copy(&unumchars)) {
                    Ok(ok__) => {
                        ppescc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptDebug64 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptDebug64 {}
windows_core::imp::define_interface!(IActiveScriptEncode, IActiveScriptEncode_Vtbl, 0xbb1a2ae3_a4f9_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScriptEncode, windows_core::IUnknown);
impl IActiveScriptEncode {
    pub unsafe fn EncodeSection<P0>(&self, pchin: P0, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EncodeSection)(windows_core::Interface::as_raw(self), pchin.param().abi(), cchin, core::mem::transmute(pchout), cchout, pcchret as _).ok() }
    }
    pub unsafe fn DecodeScript<P0>(&self, pchin: P0, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DecodeScript)(windows_core::Interface::as_raw(self), pchin.param().abi(), cchin, core::mem::transmute(pchout), cchout, pcchret as _).ok() }
    }
    pub unsafe fn GetEncodeProgId(&self, pbstrout: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEncodeProgId)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrout)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EncodeSection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub DecodeScript: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetEncodeProgId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptEncode_Impl: windows_core::IUnknownImpl {
    fn EncodeSection(&self, pchin: &windows_core::PCWSTR, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::Result<()>;
    fn DecodeScript(&self, pchin: &windows_core::PCWSTR, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::Result<()>;
    fn GetEncodeProgId(&self, pbstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IActiveScriptEncode_Vtbl {
    pub const fn new<Identity: IActiveScriptEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncodeSection<Identity: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchin: windows_core::PCWSTR, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptEncode_Impl::EncodeSection(this, core::mem::transmute(&pchin), core::mem::transmute_copy(&cchin), core::mem::transmute_copy(&pchout), core::mem::transmute_copy(&cchout), core::mem::transmute_copy(&pcchret)).into()
            }
        }
        unsafe extern "system" fn DecodeScript<Identity: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchin: windows_core::PCWSTR, cchin: u32, pchout: windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptEncode_Impl::DecodeScript(this, core::mem::transmute(&pchin), core::mem::transmute_copy(&cchin), core::mem::transmute_copy(&pchout), core::mem::transmute_copy(&cchout), core::mem::transmute_copy(&pcchret)).into()
            }
        }
        unsafe extern "system" fn GetEncodeProgId<Identity: IActiveScriptEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptEncode_Impl::GetEncodeProgId(this, core::mem::transmute_copy(&pbstrout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EncodeSection: EncodeSection::<Identity, OFFSET>,
            DecodeScript: DecodeScript::<Identity, OFFSET>,
            GetEncodeProgId: GetEncodeProgId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptEncode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptEncode {}
windows_core::imp::define_interface!(IActiveScriptError, IActiveScriptError_Vtbl, 0xeae1ba61_a4ed_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScriptError, windows_core::IUnknown);
impl IActiveScriptError {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetExceptionInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pexcepinfo)).ok() }
    }
    pub unsafe fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSourcePosition)(windows_core::Interface::as_raw(self), pdwsourcecontext as _, pullinenumber as _, plcharacterposition as _).ok() }
    }
    pub unsafe fn GetSourceLineText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceLineText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExceptionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExceptionInfo: usize,
    pub GetSourcePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut i32) -> windows_core::HRESULT,
    pub GetSourceLineText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError_Impl: windows_core::IUnknownImpl {
    fn GetExceptionInfo(&self, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
    fn GetSourcePosition(&self, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::Result<()>;
    fn GetSourceLineText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptError_Vtbl {
    pub const fn new<Identity: IActiveScriptError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExceptionInfo<Identity: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptError_Impl::GetExceptionInfo(this, core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        unsafe extern "system" fn GetSourcePosition<Identity: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptError_Impl::GetSourcePosition(this, core::mem::transmute_copy(&pdwsourcecontext), core::mem::transmute_copy(&pullinenumber), core::mem::transmute_copy(&plcharacterposition)).into()
            }
        }
        unsafe extern "system" fn GetSourceLineText<Identity: IActiveScriptError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsourceline: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptError_Impl::GetSourceLineText(this) {
                    Ok(ok__) => {
                        pbstrsourceline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetExceptionInfo: GetExceptionInfo::<Identity, OFFSET>,
            GetSourcePosition: GetSourcePosition::<Identity, OFFSET>,
            GetSourceLineText: GetSourceLineText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptError {}
windows_core::imp::define_interface!(IActiveScriptError64, IActiveScriptError64_Vtbl, 0xb21fb2a1_5b8f_4963_8c21_21450f84ed7f);
impl core::ops::Deref for IActiveScriptError64 {
    type Target = IActiveScriptError;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptError64, windows_core::IUnknown, IActiveScriptError);
impl IActiveScriptError64 {
    pub unsafe fn GetSourcePosition64(&self, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSourcePosition64)(windows_core::Interface::as_raw(self), pdwsourcecontext as _, pullinenumber as _, plcharacterposition as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptError64_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetSourcePosition64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError64_Impl: IActiveScriptError_Impl {
    fn GetSourcePosition64(&self, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptError64_Vtbl {
    pub const fn new<Identity: IActiveScriptError64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourcePosition64<Identity: IActiveScriptError64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptError64_Impl::GetSourcePosition64(this, core::mem::transmute_copy(&pdwsourcecontext), core::mem::transmute_copy(&pullinenumber), core::mem::transmute_copy(&plcharacterposition)).into()
            }
        }
        Self { base__: IActiveScriptError_Vtbl::new::<Identity, OFFSET>(), GetSourcePosition64: GetSourcePosition64::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptError64 as windows_core::Interface>::IID || iid == &<IActiveScriptError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptError64 {}
windows_core::imp::define_interface!(IActiveScriptErrorDebug, IActiveScriptErrorDebug_Vtbl, 0x51973c12_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IActiveScriptErrorDebug {
    type Target = IActiveScriptError;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptErrorDebug, windows_core::IUnknown, IActiveScriptError);
impl IActiveScriptErrorDebug {
    pub unsafe fn GetDocumentContext(&self) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStackFrame(&self) -> windows_core::Result<IDebugStackFrame> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStackFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptErrorDebug_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetDocumentContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStackFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptErrorDebug_Impl: IActiveScriptError_Impl {
    fn GetDocumentContext(&self) -> windows_core::Result<IDebugDocumentContext>;
    fn GetStackFrame(&self) -> windows_core::Result<IDebugStackFrame>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptErrorDebug_Vtbl {
    pub const fn new<Identity: IActiveScriptErrorDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentContext<Identity: IActiveScriptErrorDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppssc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptErrorDebug_Impl::GetDocumentContext(this) {
                    Ok(ok__) => {
                        ppssc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStackFrame<Identity: IActiveScriptErrorDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptErrorDebug_Impl::GetStackFrame(this) {
                    Ok(ok__) => {
                        ppdsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IActiveScriptError_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContext: GetDocumentContext::<Identity, OFFSET>,
            GetStackFrame: GetStackFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptErrorDebug as windows_core::Interface>::IID || iid == &<IActiveScriptError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptErrorDebug {}
windows_core::imp::define_interface!(IActiveScriptErrorDebug110, IActiveScriptErrorDebug110_Vtbl, 0x516e42b6_89a8_4530_937b_5f0708431442);
windows_core::imp::interface_hierarchy!(IActiveScriptErrorDebug110, windows_core::IUnknown);
impl IActiveScriptErrorDebug110 {
    pub unsafe fn GetExceptionThrownKind(&self) -> windows_core::Result<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExceptionThrownKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptErrorDebug110_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetExceptionThrownKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND) -> windows_core::HRESULT,
}
pub trait IActiveScriptErrorDebug110_Impl: windows_core::IUnknownImpl {
    fn GetExceptionThrownKind(&self) -> windows_core::Result<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND>;
}
impl IActiveScriptErrorDebug110_Vtbl {
    pub const fn new<Identity: IActiveScriptErrorDebug110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExceptionThrownKind<Identity: IActiveScriptErrorDebug110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexceptionkind: *mut SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptErrorDebug110_Impl::GetExceptionThrownKind(this) {
                    Ok(ok__) => {
                        pexceptionkind.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetExceptionThrownKind: GetExceptionThrownKind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptErrorDebug110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptErrorDebug110 {}
windows_core::imp::define_interface!(IActiveScriptGarbageCollector, IActiveScriptGarbageCollector_Vtbl, 0x6aa2c4a0_2b53_11d4_a2a0_00104bd35090);
windows_core::imp::interface_hierarchy!(IActiveScriptGarbageCollector, windows_core::IUnknown);
impl IActiveScriptGarbageCollector {
    pub unsafe fn CollectGarbage(&self, scriptgctype: SCRIPTGCTYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CollectGarbage)(windows_core::Interface::as_raw(self), scriptgctype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptGarbageCollector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CollectGarbage: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPTGCTYPE) -> windows_core::HRESULT,
}
pub trait IActiveScriptGarbageCollector_Impl: windows_core::IUnknownImpl {
    fn CollectGarbage(&self, scriptgctype: SCRIPTGCTYPE) -> windows_core::Result<()>;
}
impl IActiveScriptGarbageCollector_Vtbl {
    pub const fn new<Identity: IActiveScriptGarbageCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CollectGarbage<Identity: IActiveScriptGarbageCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptgctype: SCRIPTGCTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptGarbageCollector_Impl::CollectGarbage(this, core::mem::transmute_copy(&scriptgctype)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CollectGarbage: CollectGarbage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptGarbageCollector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptGarbageCollector {}
windows_core::imp::define_interface!(IActiveScriptHostEncode, IActiveScriptHostEncode_Vtbl, 0xbee9b76e_cfe3_11d1_b747_00c04fc2b085);
windows_core::imp::interface_hierarchy!(IActiveScriptHostEncode, windows_core::IUnknown);
impl IActiveScriptHostEncode {
    pub unsafe fn EncodeScriptHostFile(&self, bstrinfile: &windows_core::BSTR, pbstroutfile: *mut windows_core::BSTR, cflags: u32, bstrdefaultlang: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EncodeScriptHostFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrinfile), core::mem::transmute(pbstroutfile), cflags, core::mem::transmute_copy(bstrdefaultlang)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptHostEncode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EncodeScriptHostFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptHostEncode_Impl: windows_core::IUnknownImpl {
    fn EncodeScriptHostFile(&self, bstrinfile: &windows_core::BSTR, pbstroutfile: *mut windows_core::BSTR, cflags: u32, bstrdefaultlang: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IActiveScriptHostEncode_Vtbl {
    pub const fn new<Identity: IActiveScriptHostEncode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncodeScriptHostFile<Identity: IActiveScriptHostEncode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinfile: *mut core::ffi::c_void, pbstroutfile: *mut *mut core::ffi::c_void, cflags: u32, bstrdefaultlang: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptHostEncode_Impl::EncodeScriptHostFile(this, core::mem::transmute(&bstrinfile), core::mem::transmute_copy(&pbstroutfile), core::mem::transmute_copy(&cflags), core::mem::transmute(&bstrdefaultlang)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EncodeScriptHostFile: EncodeScriptHostFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptHostEncode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptHostEncode {}
windows_core::imp::define_interface!(IActiveScriptParse32, IActiveScriptParse32_Vtbl, 0xbb1a2ae2_a4f9_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScriptParse32, windows_core::IUnknown);
impl IActiveScriptParse32 {
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pstrdefaultname: P0, pstrcode: P1, pstritemname: P2, pstrsubitemname: P3, pstreventname: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddScriptlet)(windows_core::Interface::as_raw(self), pstrdefaultname.param().abi(), pstrcode.param().abi(), pstritemname.param().abi(), pstrsubitemname.param().abi(), pstreventname.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, core::mem::transmute(pbstrname), core::mem::transmute(pexcepinfo)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ParseScriptText<P0, P1, P2, P3>(&self, pstrcode: P0, pstritemname: P1, punkcontext: P2, pstrdelimiter: P3, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseScriptText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParse32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddScriptlet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, u32, *mut *mut core::ffi::c_void, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddScriptlet: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ParseScriptText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut super::super::super::Variant::VARIANT, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ParseScriptText: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse32_Impl: windows_core::IUnknownImpl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn AddScriptlet(&self, pstrdefaultname: &windows_core::PCWSTR, pstrcode: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, pstrsubitemname: &windows_core::PCWSTR, pstreventname: &windows_core::PCWSTR, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
    fn ParseScriptText(&self, pstrcode: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptParse32_Vtbl {
    pub const fn new<Identity: IActiveScriptParse32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitNew<Identity: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse32_Impl::InitNew(this).into()
            }
        }
        unsafe extern "system" fn AddScriptlet<Identity: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdefaultname: windows_core::PCWSTR, pstrcode: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, pstrsubitemname: windows_core::PCWSTR, pstreventname: windows_core::PCWSTR, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut *mut core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse32_Impl::AddScriptlet(this, core::mem::transmute(&pstrdefaultname), core::mem::transmute(&pstrcode), core::mem::transmute(&pstritemname), core::mem::transmute(&pstrsubitemname), core::mem::transmute(&pstreventname), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        unsafe extern "system" fn ParseScriptText<Identity: IActiveScriptParse32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse32_Impl::ParseScriptText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParse32 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IActiveScriptParse32 {}
windows_core::imp::define_interface!(IActiveScriptParse64, IActiveScriptParse64_Vtbl, 0xc7ef7658_e1ee_480e_97ea_d52cb4d76d17);
windows_core::imp::interface_hierarchy!(IActiveScriptParse64, windows_core::IUnknown);
impl IActiveScriptParse64 {
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddScriptlet<P0, P1, P2, P3, P4, P5>(&self, pstrdefaultname: P0, pstrcode: P1, pstritemname: P2, pstrsubitemname: P3, pstreventname: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddScriptlet)(windows_core::Interface::as_raw(self), pstrdefaultname.param().abi(), pstrcode.param().abi(), pstritemname.param().abi(), pstrsubitemname.param().abi(), pstreventname.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, core::mem::transmute(pbstrname), core::mem::transmute(pexcepinfo)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ParseScriptText<P0, P1, P2, P3>(&self, pstrcode: P0, pstritemname: P1, punkcontext: P2, pstrdelimiter: P3, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseScriptText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParse64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddScriptlet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u64, u32, u32, *mut *mut core::ffi::c_void, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddScriptlet: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ParseScriptText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u32, u32, *mut super::super::super::Variant::VARIANT, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ParseScriptText: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse64_Impl: windows_core::IUnknownImpl {
    fn InitNew(&self) -> windows_core::Result<()>;
    fn AddScriptlet(&self, pstrdefaultname: &windows_core::PCWSTR, pstrcode: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, pstrsubitemname: &windows_core::PCWSTR, pstreventname: &windows_core::PCWSTR, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
    fn ParseScriptText(&self, pstrcode: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptParse64_Vtbl {
    pub const fn new<Identity: IActiveScriptParse64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitNew<Identity: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse64_Impl::InitNew(this).into()
            }
        }
        unsafe extern "system" fn AddScriptlet<Identity: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdefaultname: windows_core::PCWSTR, pstrcode: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, pstrsubitemname: windows_core::PCWSTR, pstreventname: windows_core::PCWSTR, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut *mut core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse64_Impl::AddScriptlet(this, core::mem::transmute(&pstrdefaultname), core::mem::transmute(&pstrcode), core::mem::transmute(&pstritemname), core::mem::transmute(&pstrsubitemname), core::mem::transmute(&pstreventname), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        unsafe extern "system" fn ParseScriptText<Identity: IActiveScriptParse64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptParse64_Impl::ParseScriptText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitNew: InitNew::<Identity, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParse64 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IActiveScriptParse64 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedure2_32, IActiveScriptParseProcedure2_32_Vtbl, 0x71ee5b20_fb04_11d1_b3a8_00a0c911e8b2);
impl core::ops::Deref for IActiveScriptParseProcedure2_32 {
    type Target = IActiveScriptParseProcedure32;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedure2_32, windows_core::IUnknown, IActiveScriptParseProcedure32);
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure2_32_Vtbl {
    pub base__: IActiveScriptParseProcedure32_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_32_Impl: IActiveScriptParseProcedure32_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure2_32_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedure2_32_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IActiveScriptParseProcedure32_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedure2_32 as windows_core::Interface>::IID || iid == &<IActiveScriptParseProcedure32 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedure2_32 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedure2_64, IActiveScriptParseProcedure2_64_Vtbl, 0xfe7c4271_210c_448d_9f54_76dab7047b28);
impl core::ops::Deref for IActiveScriptParseProcedure2_64 {
    type Target = IActiveScriptParseProcedure64;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedure2_64, windows_core::IUnknown, IActiveScriptParseProcedure64);
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure2_64_Vtbl {
    pub base__: IActiveScriptParseProcedure64_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_64_Impl: IActiveScriptParseProcedure64_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure2_64_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedure2_64_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IActiveScriptParseProcedure64_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedure2_64 as windows_core::Interface>::IID || iid == &<IActiveScriptParseProcedure64 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedure2_64 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedure32, IActiveScriptParseProcedure32_Vtbl, 0xaa5b6a80_b834_11d0_932f_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedure32, windows_core::IUnknown);
impl IActiveScriptParseProcedure32 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseProcedureText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstrformalparams.param().abi(), pstrprocedurename.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure32_Impl: windows_core::IUnknownImpl {
    fn ParseProcedureText(&self, pstrcode: &windows_core::PCWSTR, pstrformalparams: &windows_core::PCWSTR, pstrprocedurename: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure32_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedure32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseProcedureText<Identity: IActiveScriptParseProcedure32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstrformalparams: windows_core::PCWSTR, pstrprocedurename: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptParseProcedure32_Impl::ParseProcedureText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstrformalparams), core::mem::transmute(&pstrprocedurename), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedure32 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedure32 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedure64, IActiveScriptParseProcedure64_Vtbl, 0xc64713b6_e029_4cc5_9200_438b72890b6a);
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedure64, windows_core::IUnknown);
impl IActiveScriptParseProcedure64 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4, P5>(&self, pstrcode: P0, pstrformalparams: P1, pstrprocedurename: P2, pstritemname: P3, punkcontext: P4, pstrdelimiter: P5, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseProcedureText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstrformalparams.param().abi(), pstrprocedurename.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedure64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure64_Impl: windows_core::IUnknownImpl {
    fn ParseProcedureText(&self, pstrcode: &windows_core::PCWSTR, pstrformalparams: &windows_core::PCWSTR, pstrprocedurename: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedure64_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedure64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseProcedureText<Identity: IActiveScriptParseProcedure64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstrformalparams: windows_core::PCWSTR, pstrprocedurename: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptParseProcedure64_Impl::ParseProcedureText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstrformalparams), core::mem::transmute(&pstrprocedurename), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedure64 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedure64 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedureOld32, IActiveScriptParseProcedureOld32_Vtbl, 0x1cff0050_6fdd_11d0_9328_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedureOld32, windows_core::IUnknown);
impl IActiveScriptParseProcedureOld32 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4>(&self, pstrcode: P0, pstrformalparams: P1, pstritemname: P2, punkcontext: P3, pstrdelimiter: P4, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseProcedureText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstrformalparams.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedureOld32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld32_Impl: windows_core::IUnknownImpl {
    fn ParseProcedureText(&self, pstrcode: &windows_core::PCWSTR, pstrformalparams: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedureOld32_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedureOld32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseProcedureText<Identity: IActiveScriptParseProcedureOld32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstrformalparams: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptParseProcedureOld32_Impl::ParseProcedureText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstrformalparams), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedureOld32 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedureOld32 {}
windows_core::imp::define_interface!(IActiveScriptParseProcedureOld64, IActiveScriptParseProcedureOld64_Vtbl, 0x21f57128_08c9_4638_ba12_22d15d88dc5c);
windows_core::imp::interface_hierarchy!(IActiveScriptParseProcedureOld64, windows_core::IUnknown);
impl IActiveScriptParseProcedureOld64 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseProcedureText<P0, P1, P2, P3, P4>(&self, pstrcode: P0, pstrformalparams: P1, pstritemname: P2, punkcontext: P3, pstrdelimiter: P4, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseProcedureText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), pstrformalparams.param().abi(), pstritemname.param().abi(), punkcontext.param().abi(), pstrdelimiter.param().abi(), dwsourcecontextcookie, ulstartinglinenumber, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptParseProcedureOld64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseProcedureText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseProcedureText: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld64_Impl: windows_core::IUnknownImpl {
    fn ParseProcedureText(&self, pstrcode: &windows_core::PCWSTR, pstrformalparams: &windows_core::PCWSTR, pstritemname: &windows_core::PCWSTR, punkcontext: windows_core::Ref<'_, windows_core::IUnknown>, pstrdelimiter: &windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptParseProcedureOld64_Vtbl {
    pub const fn new<Identity: IActiveScriptParseProcedureOld64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseProcedureText<Identity: IActiveScriptParseProcedureOld64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, pstrformalparams: windows_core::PCWSTR, pstritemname: windows_core::PCWSTR, punkcontext: *mut core::ffi::c_void, pstrdelimiter: windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptParseProcedureOld64_Impl::ParseProcedureText(this, core::mem::transmute(&pstrcode), core::mem::transmute(&pstrformalparams), core::mem::transmute(&pstritemname), core::mem::transmute_copy(&punkcontext), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwsourcecontextcookie), core::mem::transmute_copy(&ulstartinglinenumber), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ParseProcedureText: ParseProcedureText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptParseProcedureOld64 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptParseProcedureOld64 {}
windows_core::imp::define_interface!(IActiveScriptProfilerCallback, IActiveScriptProfilerCallback_Vtbl, 0x740eca23_7d9d_42e5_ba9d_f8b24b1c7a9b);
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerCallback, windows_core::IUnknown);
impl IActiveScriptProfilerCallback {
    pub unsafe fn Initialize(&self, dwcontext: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), dwcontext).ok() }
    }
    pub unsafe fn Shutdown(&self, hrreason: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), hrreason).ok() }
    }
    pub unsafe fn ScriptCompiled<P2>(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ScriptCompiled)(windows_core::Interface::as_raw(self), scriptid, r#type, pidebugdocumentcontext.param().abi()).ok() }
    }
    pub unsafe fn FunctionCompiled<P2, P3, P4>(&self, functionid: i32, scriptid: i32, pwszfunctionname: P2, pwszfunctionnamehint: P3, pidebugdocumentcontext: P4) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FunctionCompiled)(windows_core::Interface::as_raw(self), functionid, scriptid, pwszfunctionname.param().abi(), pwszfunctionnamehint.param().abi(), pidebugdocumentcontext.param().abi()).ok() }
    }
    pub unsafe fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnFunctionEnter)(windows_core::Interface::as_raw(self), scriptid, functionid).ok() }
    }
    pub unsafe fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnFunctionExit)(windows_core::Interface::as_raw(self), scriptid, functionid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ScriptCompiled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, PROFILER_SCRIPT_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FunctionCompiled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnFunctionEnter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub OnFunctionExit: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerCallback_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, dwcontext: u32) -> windows_core::Result<()>;
    fn Shutdown(&self, hrreason: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ScriptCompiled(&self, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn FunctionCompiled(&self, functionid: i32, scriptid: i32, pwszfunctionname: &windows_core::PCWSTR, pwszfunctionnamehint: &windows_core::PCWSTR, pidebugdocumentcontext: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnFunctionEnter(&self, scriptid: i32, functionid: i32) -> windows_core::Result<()>;
    fn OnFunctionExit(&self, scriptid: i32, functionid: i32) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerCallback_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::Initialize(this, core::mem::transmute_copy(&dwcontext)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrreason: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::Shutdown(this, core::mem::transmute_copy(&hrreason)).into()
            }
        }
        unsafe extern "system" fn ScriptCompiled<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::ScriptCompiled(this, core::mem::transmute_copy(&scriptid), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pidebugdocumentcontext)).into()
            }
        }
        unsafe extern "system" fn FunctionCompiled<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: i32, scriptid: i32, pwszfunctionname: windows_core::PCWSTR, pwszfunctionnamehint: windows_core::PCWSTR, pidebugdocumentcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::FunctionCompiled(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&scriptid), core::mem::transmute(&pwszfunctionname), core::mem::transmute(&pwszfunctionnamehint), core::mem::transmute_copy(&pidebugdocumentcontext)).into()
            }
        }
        unsafe extern "system" fn OnFunctionEnter<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptid: i32, functionid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::OnFunctionEnter(this, core::mem::transmute_copy(&scriptid), core::mem::transmute_copy(&functionid)).into()
            }
        }
        unsafe extern "system" fn OnFunctionExit<Identity: IActiveScriptProfilerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scriptid: i32, functionid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback_Impl::OnFunctionExit(this, core::mem::transmute_copy(&scriptid), core::mem::transmute_copy(&functionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            ScriptCompiled: ScriptCompiled::<Identity, OFFSET>,
            FunctionCompiled: FunctionCompiled::<Identity, OFFSET>,
            OnFunctionEnter: OnFunctionEnter::<Identity, OFFSET>,
            OnFunctionExit: OnFunctionExit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerCallback {}
windows_core::imp::define_interface!(IActiveScriptProfilerCallback2, IActiveScriptProfilerCallback2_Vtbl, 0x31b7f8ad_a637_409c_b22f_040995b6103d);
impl core::ops::Deref for IActiveScriptProfilerCallback2 {
    type Target = IActiveScriptProfilerCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerCallback2, windows_core::IUnknown, IActiveScriptProfilerCallback);
impl IActiveScriptProfilerCallback2 {
    pub unsafe fn OnFunctionEnterByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFunctionEnterByName)(windows_core::Interface::as_raw(self), pwszfunctionname.param().abi(), r#type).ok() }
    }
    pub unsafe fn OnFunctionExitByName<P0>(&self, pwszfunctionname: P0, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFunctionExitByName)(windows_core::Interface::as_raw(self), pwszfunctionname.param().abi(), r#type).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback2_Vtbl {
    pub base__: IActiveScriptProfilerCallback_Vtbl,
    pub OnFunctionEnterByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PROFILER_SCRIPT_TYPE) -> windows_core::HRESULT,
    pub OnFunctionExitByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PROFILER_SCRIPT_TYPE) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerCallback2_Impl: IActiveScriptProfilerCallback_Impl {
    fn OnFunctionEnterByName(&self, pwszfunctionname: &windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::Result<()>;
    fn OnFunctionExitByName(&self, pwszfunctionname: &windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerCallback2_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFunctionEnterByName<Identity: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfunctionname: windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback2_Impl::OnFunctionEnterByName(this, core::mem::transmute(&pwszfunctionname), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn OnFunctionExitByName<Identity: IActiveScriptProfilerCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfunctionname: windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback2_Impl::OnFunctionExitByName(this, core::mem::transmute(&pwszfunctionname), core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: IActiveScriptProfilerCallback_Vtbl::new::<Identity, OFFSET>(),
            OnFunctionEnterByName: OnFunctionEnterByName::<Identity, OFFSET>,
            OnFunctionExitByName: OnFunctionExitByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerCallback2 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerCallback2 {}
windows_core::imp::define_interface!(IActiveScriptProfilerCallback3, IActiveScriptProfilerCallback3_Vtbl, 0x6ac5ad25_2037_4687_91df_b59979d93d73);
impl core::ops::Deref for IActiveScriptProfilerCallback3 {
    type Target = IActiveScriptProfilerCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerCallback3, windows_core::IUnknown, IActiveScriptProfilerCallback, IActiveScriptProfilerCallback2);
impl IActiveScriptProfilerCallback3 {
    pub unsafe fn SetWebWorkerId(&self, webworkerid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWebWorkerId)(windows_core::Interface::as_raw(self), webworkerid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerCallback3_Vtbl {
    pub base__: IActiveScriptProfilerCallback2_Vtbl,
    pub SetWebWorkerId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerCallback3_Impl: IActiveScriptProfilerCallback2_Impl {
    fn SetWebWorkerId(&self, webworkerid: u32) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerCallback3_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerCallback3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetWebWorkerId<Identity: IActiveScriptProfilerCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, webworkerid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerCallback3_Impl::SetWebWorkerId(this, core::mem::transmute_copy(&webworkerid)).into()
            }
        }
        Self { base__: IActiveScriptProfilerCallback2_Vtbl::new::<Identity, OFFSET>(), SetWebWorkerId: SetWebWorkerId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerCallback3 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerCallback as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerCallback2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerCallback3 {}
windows_core::imp::define_interface!(IActiveScriptProfilerControl, IActiveScriptProfilerControl_Vtbl, 0x784b5ff0_69b0_47d1_a7dc_2518f4230e90);
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerControl, windows_core::IUnknown);
impl IActiveScriptProfilerControl {
    pub unsafe fn StartProfiling(&self, clsidprofilerobject: *const windows_core::GUID, dweventmask: u32, dwcontext: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartProfiling)(windows_core::Interface::as_raw(self), clsidprofilerobject, dweventmask, dwcontext).ok() }
    }
    pub unsafe fn SetProfilerEventMask(&self, dweventmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProfilerEventMask)(windows_core::Interface::as_raw(self), dweventmask).ok() }
    }
    pub unsafe fn StopProfiling(&self, hrshutdownreason: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopProfiling)(windows_core::Interface::as_raw(self), hrshutdownreason).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartProfiling: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32) -> windows_core::HRESULT,
    pub SetProfilerEventMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StopProfiling: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerControl_Impl: windows_core::IUnknownImpl {
    fn StartProfiling(&self, clsidprofilerobject: *const windows_core::GUID, dweventmask: u32, dwcontext: u32) -> windows_core::Result<()>;
    fn SetProfilerEventMask(&self, dweventmask: u32) -> windows_core::Result<()>;
    fn StopProfiling(&self, hrshutdownreason: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerControl_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartProfiling<Identity: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidprofilerobject: *const windows_core::GUID, dweventmask: u32, dwcontext: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl_Impl::StartProfiling(this, core::mem::transmute_copy(&clsidprofilerobject), core::mem::transmute_copy(&dweventmask), core::mem::transmute_copy(&dwcontext)).into()
            }
        }
        unsafe extern "system" fn SetProfilerEventMask<Identity: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl_Impl::SetProfilerEventMask(this, core::mem::transmute_copy(&dweventmask)).into()
            }
        }
        unsafe extern "system" fn StopProfiling<Identity: IActiveScriptProfilerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrshutdownreason: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl_Impl::StopProfiling(this, core::mem::transmute_copy(&hrshutdownreason)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartProfiling: StartProfiling::<Identity, OFFSET>,
            SetProfilerEventMask: SetProfilerEventMask::<Identity, OFFSET>,
            StopProfiling: StopProfiling::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerControl {}
windows_core::imp::define_interface!(IActiveScriptProfilerControl2, IActiveScriptProfilerControl2_Vtbl, 0x47810165_498f_40be_94f1_653557e9e7da);
impl core::ops::Deref for IActiveScriptProfilerControl2 {
    type Target = IActiveScriptProfilerControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerControl2, windows_core::IUnknown, IActiveScriptProfilerControl);
impl IActiveScriptProfilerControl2 {
    pub unsafe fn CompleteProfilerStart(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteProfilerStart)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn PrepareProfilerStop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PrepareProfilerStop)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl2_Vtbl {
    pub base__: IActiveScriptProfilerControl_Vtbl,
    pub CompleteProfilerStart: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareProfilerStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerControl2_Impl: IActiveScriptProfilerControl_Impl {
    fn CompleteProfilerStart(&self) -> windows_core::Result<()>;
    fn PrepareProfilerStop(&self) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerControl2_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteProfilerStart<Identity: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl2_Impl::CompleteProfilerStart(this).into()
            }
        }
        unsafe extern "system" fn PrepareProfilerStop<Identity: IActiveScriptProfilerControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl2_Impl::PrepareProfilerStop(this).into()
            }
        }
        Self {
            base__: IActiveScriptProfilerControl_Vtbl::new::<Identity, OFFSET>(),
            CompleteProfilerStart: CompleteProfilerStart::<Identity, OFFSET>,
            PrepareProfilerStop: PrepareProfilerStop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerControl2 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerControl2 {}
windows_core::imp::define_interface!(IActiveScriptProfilerControl3, IActiveScriptProfilerControl3_Vtbl, 0x0b403015_f381_4023_a5d0_6fed076de716);
impl core::ops::Deref for IActiveScriptProfilerControl3 {
    type Target = IActiveScriptProfilerControl2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerControl3, windows_core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2);
impl IActiveScriptProfilerControl3 {
    pub unsafe fn EnumHeap(&self) -> windows_core::Result<IActiveScriptProfilerHeapEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumHeap)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl3_Vtbl {
    pub base__: IActiveScriptProfilerControl2_Vtbl,
    pub EnumHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerControl3_Impl: IActiveScriptProfilerControl2_Impl {
    fn EnumHeap(&self) -> windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl IActiveScriptProfilerControl3_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerControl3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumHeap<Identity: IActiveScriptProfilerControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptProfilerControl3_Impl::EnumHeap(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IActiveScriptProfilerControl2_Vtbl::new::<Identity, OFFSET>(), EnumHeap: EnumHeap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerControl3 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerControl3 {}
windows_core::imp::define_interface!(IActiveScriptProfilerControl4, IActiveScriptProfilerControl4_Vtbl, 0x160f94fd_9dbc_40d4_9eac_2b71db3132f4);
impl core::ops::Deref for IActiveScriptProfilerControl4 {
    type Target = IActiveScriptProfilerControl3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerControl4, windows_core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2, IActiveScriptProfilerControl3);
impl IActiveScriptProfilerControl4 {
    pub unsafe fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SummarizeHeap)(windows_core::Interface::as_raw(self), heapsummary as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl4_Vtbl {
    pub base__: IActiveScriptProfilerControl3_Vtbl,
    pub SummarizeHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROFILER_HEAP_SUMMARY) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerControl4_Impl: IActiveScriptProfilerControl3_Impl {
    fn SummarizeHeap(&self, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerControl4_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerControl4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SummarizeHeap<Identity: IActiveScriptProfilerControl4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerControl4_Impl::SummarizeHeap(this, core::mem::transmute_copy(&heapsummary)).into()
            }
        }
        Self { base__: IActiveScriptProfilerControl3_Vtbl::new::<Identity, OFFSET>(), SummarizeHeap: SummarizeHeap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerControl4 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl2 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerControl4 {}
windows_core::imp::define_interface!(IActiveScriptProfilerControl5, IActiveScriptProfilerControl5_Vtbl, 0x1c01a2d1_8f0f_46a5_9720_0d7ed2c62f0a);
impl core::ops::Deref for IActiveScriptProfilerControl5 {
    type Target = IActiveScriptProfilerControl4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerControl5, windows_core::IUnknown, IActiveScriptProfilerControl, IActiveScriptProfilerControl2, IActiveScriptProfilerControl3, IActiveScriptProfilerControl4);
impl IActiveScriptProfilerControl5 {
    pub unsafe fn EnumHeap2(&self, enumflags: PROFILER_HEAP_ENUM_FLAGS) -> windows_core::Result<IActiveScriptProfilerHeapEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumHeap2)(windows_core::Interface::as_raw(self), enumflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerControl5_Vtbl {
    pub base__: IActiveScriptProfilerControl4_Vtbl,
    pub EnumHeap2: unsafe extern "system" fn(*mut core::ffi::c_void, PROFILER_HEAP_ENUM_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerControl5_Impl: IActiveScriptProfilerControl4_Impl {
    fn EnumHeap2(&self, enumflags: PROFILER_HEAP_ENUM_FLAGS) -> windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl IActiveScriptProfilerControl5_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerControl5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumHeap2<Identity: IActiveScriptProfilerControl5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumflags: PROFILER_HEAP_ENUM_FLAGS, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptProfilerControl5_Impl::EnumHeap2(this, core::mem::transmute_copy(&enumflags)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IActiveScriptProfilerControl4_Vtbl::new::<Identity, OFFSET>(), EnumHeap2: EnumHeap2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerControl5 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl2 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl3 as windows_core::Interface>::IID || iid == &<IActiveScriptProfilerControl4 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerControl5 {}
windows_core::imp::define_interface!(IActiveScriptProfilerHeapEnum, IActiveScriptProfilerHeapEnum_Vtbl, 0x32e4694e_0d37_419b_b93d_fa20ded6e8ea);
windows_core::imp::interface_hierarchy!(IActiveScriptProfilerHeapEnum, windows_core::IUnknown);
impl IActiveScriptProfilerHeapEnum {
    pub unsafe fn Next(&self, heapobjects: &mut [*mut PROFILER_HEAP_OBJECT], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), heapobjects.len().try_into().unwrap(), core::mem::transmute(heapobjects.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn GetOptionalInfo(&self, heapobject: *const PROFILER_HEAP_OBJECT, optionalinfo: &mut [PROFILER_HEAP_OBJECT_OPTIONAL_INFO]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOptionalInfo)(windows_core::Interface::as_raw(self), heapobject, optionalinfo.len().try_into().unwrap(), core::mem::transmute(optionalinfo.as_ptr())).ok() }
    }
    pub unsafe fn FreeObjectAndOptionalInfo(&self, heapobjects: &[*const PROFILER_HEAP_OBJECT]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeObjectAndOptionalInfo)(windows_core::Interface::as_raw(self), heapobjects.len().try_into().unwrap(), core::mem::transmute(heapobjects.as_ptr())).ok() }
    }
    pub unsafe fn GetNameIdMap(&self, pnamelist: *mut *mut *mut windows_core::PCWSTR, pcelt: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNameIdMap)(windows_core::Interface::as_raw(self), pnamelist as _, pcelt as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProfilerHeapEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut PROFILER_HEAP_OBJECT, *mut u32) -> windows_core::HRESULT,
    pub GetOptionalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROFILER_HEAP_OBJECT, u32, *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> windows_core::HRESULT,
    pub FreeObjectAndOptionalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const PROFILER_HEAP_OBJECT) -> windows_core::HRESULT,
    pub GetNameIdMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
}
pub trait IActiveScriptProfilerHeapEnum_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn GetOptionalInfo(&self, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> windows_core::Result<()>;
    fn FreeObjectAndOptionalInfo(&self, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> windows_core::Result<()>;
    fn GetNameIdMap(&self, pnamelist: *mut *mut *mut windows_core::PCWSTR, pcelt: *mut u32) -> windows_core::Result<()>;
}
impl IActiveScriptProfilerHeapEnum_Vtbl {
    pub const fn new<Identity: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerHeapEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&heapobjects), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn GetOptionalInfo<Identity: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerHeapEnum_Impl::GetOptionalInfo(this, core::mem::transmute_copy(&heapobject), core::mem::transmute_copy(&celt), core::mem::transmute_copy(&optionalinfo)).into()
            }
        }
        unsafe extern "system" fn FreeObjectAndOptionalInfo<Identity: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerHeapEnum_Impl::FreeObjectAndOptionalInfo(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&heapobjects)).into()
            }
        }
        unsafe extern "system" fn GetNameIdMap<Identity: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamelist: *mut *mut *mut windows_core::PCWSTR, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProfilerHeapEnum_Impl::GetNameIdMap(this, core::mem::transmute_copy(&pnamelist), core::mem::transmute_copy(&pcelt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetOptionalInfo: GetOptionalInfo::<Identity, OFFSET>,
            FreeObjectAndOptionalInfo: FreeObjectAndOptionalInfo::<Identity, OFFSET>,
            GetNameIdMap: GetNameIdMap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProfilerHeapEnum as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptProfilerHeapEnum {}
windows_core::imp::define_interface!(IActiveScriptProperty, IActiveScriptProperty_Vtbl, 0x4954e0d0_fbc7_11d1_8410_006008c3fbfc);
windows_core::imp::interface_hierarchy!(IActiveScriptProperty, windows_core::IUnknown);
impl IActiveScriptProperty {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT) -> windows_core::Result<super::super::super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), dwproperty, core::mem::transmute(pvarindex), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), dwproperty, core::mem::transmute(pvarindex), core::mem::transmute(pvarvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Variant::VARIANT, *mut super::super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::Variant::VARIANT, *const super::super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptProperty_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT) -> windows_core::Result<super::super::super::Variant::VARIANT>;
    fn SetProperty(&self, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptProperty_Vtbl {
    pub const fn new<Identity: IActiveScriptProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IActiveScriptProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *mut super::super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptProperty_Impl::GetProperty(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&pvarindex)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IActiveScriptProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptProperty_Impl::SetProperty(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&pvarindex), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IActiveScriptProperty {}
windows_core::imp::define_interface!(IActiveScriptSIPInfo, IActiveScriptSIPInfo_Vtbl, 0x764651d0_38de_11d4_a2a3_00104bd35090);
windows_core::imp::interface_hierarchy!(IActiveScriptSIPInfo, windows_core::IUnknown);
impl IActiveScriptSIPInfo {
    pub unsafe fn GetSIPOID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSIPOID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSIPInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSIPOID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IActiveScriptSIPInfo_Impl: windows_core::IUnknownImpl {
    fn GetSIPOID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IActiveScriptSIPInfo_Vtbl {
    pub const fn new<Identity: IActiveScriptSIPInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSIPOID<Identity: IActiveScriptSIPInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poid_sip: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSIPInfo_Impl::GetSIPOID(this) {
                    Ok(ok__) => {
                        poid_sip.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSIPOID: GetSIPOID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSIPInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSIPInfo {}
windows_core::imp::define_interface!(IActiveScriptSite, IActiveScriptSite_Vtbl, 0xdb01a1e3_a42b_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScriptSite, windows_core::IUnknown);
impl IActiveScriptSite {
    pub unsafe fn GetLCID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLCID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetItemInfo<P0>(&self, pstrname: P0, dwreturnmask: u32, ppiunkitem: *mut Option<windows_core::IUnknown>, ppti: *mut Option<super::super::super::Com::ITypeInfo>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetItemInfo)(windows_core::Interface::as_raw(self), pstrname.param().abi(), dwreturnmask, core::mem::transmute(ppiunkitem), core::mem::transmute(ppti)).ok() }
    }
    pub unsafe fn GetDocVersionString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocVersionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OnScriptTerminate(&self, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnScriptTerminate)(windows_core::Interface::as_raw(self), core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo)).ok() }
    }
    pub unsafe fn OnStateChange(&self, ssscriptstate: SCRIPTSTATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStateChange)(windows_core::Interface::as_raw(self), ssscriptstate).ok() }
    }
    pub unsafe fn OnScriptError<P0>(&self, pscripterror: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptError>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnScriptError)(windows_core::Interface::as_raw(self), pscripterror.param().abi()).ok() }
    }
    pub unsafe fn OnEnterScript(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnEnterScript)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnLeaveScript(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnLeaveScript)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLCID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetItemInfo: usize,
    pub GetDocVersionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OnScriptTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Variant::VARIANT, *const super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OnScriptTerminate: usize,
    pub OnStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPTSTATE) -> windows_core::HRESULT,
    pub OnScriptError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEnterScript: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLeaveScript: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptSite_Impl: windows_core::IUnknownImpl {
    fn GetLCID(&self) -> windows_core::Result<u32>;
    fn GetItemInfo(&self, pstrname: &windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: windows_core::OutRef<'_, windows_core::IUnknown>, ppti: windows_core::OutRef<'_, super::super::super::Com::ITypeInfo>) -> windows_core::Result<()>;
    fn GetDocVersionString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OnScriptTerminate(&self, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
    fn OnStateChange(&self, ssscriptstate: SCRIPTSTATE) -> windows_core::Result<()>;
    fn OnScriptError(&self, pscripterror: windows_core::Ref<'_, IActiveScriptError>) -> windows_core::Result<()>;
    fn OnEnterScript(&self) -> windows_core::Result<()>;
    fn OnLeaveScript(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IActiveScriptSite_Vtbl {
    pub const fn new<Identity: IActiveScriptSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLCID<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSite_Impl::GetLCID(this) {
                    Ok(ok__) => {
                        plcid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemInfo<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut *mut core::ffi::c_void, ppti: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::GetItemInfo(this, core::mem::transmute(&pstrname), core::mem::transmute_copy(&dwreturnmask), core::mem::transmute_copy(&ppiunkitem), core::mem::transmute_copy(&ppti)).into()
            }
        }
        unsafe extern "system" fn GetDocVersionString<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSite_Impl::GetDocVersionString(this) {
                    Ok(ok__) => {
                        pbstrversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnScriptTerminate<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::OnScriptTerminate(this, core::mem::transmute_copy(&pvarresult), core::mem::transmute_copy(&pexcepinfo)).into()
            }
        }
        unsafe extern "system" fn OnStateChange<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ssscriptstate: SCRIPTSTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::OnStateChange(this, core::mem::transmute_copy(&ssscriptstate)).into()
            }
        }
        unsafe extern "system" fn OnScriptError<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscripterror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::OnScriptError(this, core::mem::transmute_copy(&pscripterror)).into()
            }
        }
        unsafe extern "system" fn OnEnterScript<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::OnEnterScript(this).into()
            }
        }
        unsafe extern "system" fn OnLeaveScript<Identity: IActiveScriptSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSite_Impl::OnLeaveScript(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLCID: GetLCID::<Identity, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, OFFSET>,
            GetDocVersionString: GetDocVersionString::<Identity, OFFSET>,
            OnScriptTerminate: OnScriptTerminate::<Identity, OFFSET>,
            OnStateChange: OnStateChange::<Identity, OFFSET>,
            OnScriptError: OnScriptError::<Identity, OFFSET>,
            OnEnterScript: OnEnterScript::<Identity, OFFSET>,
            OnLeaveScript: OnLeaveScript::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IActiveScriptSite {}
windows_core::imp::define_interface!(IActiveScriptSiteDebug32, IActiveScriptSiteDebug32_Vtbl, 0x51973c11_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteDebug32, windows_core::IUnknown);
impl IActiveScriptSiteDebug32 {
    pub unsafe fn GetDocumentContextFromPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentContextFromPosition)(windows_core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplication(&self) -> windows_core::Result<IDebugApplication32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnScriptErrorDebug<P0>(&self, perrordebug: P0, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptErrorDebug>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnScriptErrorDebug)(windows_core::Interface::as_raw(self), perrordebug.param().abi(), pfenterdebugger as _, pfcallonscripterrorwhencontinuing as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebug32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentContextFromPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnScriptErrorDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteDebug32_Impl: windows_core::IUnknownImpl {
    fn GetDocumentContextFromPosition(&self, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(&self) -> windows_core::Result<IDebugApplication32>;
    fn GetRootApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(&self, perrordebug: windows_core::Ref<'_, IActiveScriptErrorDebug>, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IActiveScriptSiteDebug32_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug32_Impl::GetDocumentContextFromPosition(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ucharacteroffset), core::mem::transmute_copy(&unumchars)) {
                    Ok(ok__) => {
                        ppsc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplication<Identity: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug32_Impl::GetApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdanroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug32_Impl::GetRootApplicationNode(this) {
                    Ok(ok__) => {
                        ppdanroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: IActiveScriptSiteDebug32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrordebug: *mut core::ffi::c_void, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSiteDebug32_Impl::OnScriptErrorDebug(this, core::mem::transmute_copy(&perrordebug), core::mem::transmute_copy(&pfenterdebugger), core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, OFFSET>,
            GetApplication: GetApplication::<Identity, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteDebug32 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteDebug32 {}
windows_core::imp::define_interface!(IActiveScriptSiteDebug64, IActiveScriptSiteDebug64_Vtbl, 0xd6b96b0a_7463_402c_92ac_89984226942f);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteDebug64, windows_core::IUnknown);
impl IActiveScriptSiteDebug64 {
    pub unsafe fn GetDocumentContextFromPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentContextFromPosition)(windows_core::Interface::as_raw(self), dwsourcecontext, ucharacteroffset, unumchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplication(&self) -> windows_core::Result<IDebugApplication64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnScriptErrorDebug<P0>(&self, perrordebug: P0, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptErrorDebug>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnScriptErrorDebug)(windows_core::Interface::as_raw(self), perrordebug.param().abi(), pfenterdebugger as _, pfcallonscripterrorwhencontinuing as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebug64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentContextFromPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnScriptErrorDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteDebug64_Impl: windows_core::IUnknownImpl {
    fn GetDocumentContextFromPosition(&self, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(&self) -> windows_core::Result<IDebugApplication64>;
    fn GetRootApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(&self, perrordebug: windows_core::Ref<'_, IActiveScriptErrorDebug>, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IActiveScriptSiteDebug64_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug64_Impl::GetDocumentContextFromPosition(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ucharacteroffset), core::mem::transmute_copy(&unumchars)) {
                    Ok(ok__) => {
                        ppsc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplication<Identity: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug64_Impl::GetApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdanroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebug64_Impl::GetRootApplicationNode(this) {
                    Ok(ok__) => {
                        ppdanroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: IActiveScriptSiteDebug64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrordebug: *mut core::ffi::c_void, pfenterdebugger: *mut windows_core::BOOL, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSiteDebug64_Impl::OnScriptErrorDebug(this, core::mem::transmute_copy(&perrordebug), core::mem::transmute_copy(&pfenterdebugger), core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, OFFSET>,
            GetApplication: GetApplication::<Identity, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteDebug64 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteDebug64 {}
windows_core::imp::define_interface!(IActiveScriptSiteDebugEx, IActiveScriptSiteDebugEx_Vtbl, 0xbb722ccb_6ad2_41c6_b780_af9c03ee69f5);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteDebugEx, windows_core::IUnknown);
impl IActiveScriptSiteDebugEx {
    pub unsafe fn OnCanNotJITScriptErrorDebug<P0>(&self, perrordebug: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IActiveScriptErrorDebug>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnCanNotJITScriptErrorDebug)(windows_core::Interface::as_raw(self), perrordebug.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteDebugEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCanNotJITScriptErrorDebug: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteDebugEx_Impl: windows_core::IUnknownImpl {
    fn OnCanNotJITScriptErrorDebug(&self, perrordebug: windows_core::Ref<'_, IActiveScriptErrorDebug>) -> windows_core::Result<windows_core::BOOL>;
}
impl IActiveScriptSiteDebugEx_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteDebugEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCanNotJITScriptErrorDebug<Identity: IActiveScriptSiteDebugEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrordebug: *mut core::ffi::c_void, pfcallonscripterrorwhencontinuing: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteDebugEx_Impl::OnCanNotJITScriptErrorDebug(this, core::mem::transmute_copy(&perrordebug)) {
                    Ok(ok__) => {
                        pfcallonscripterrorwhencontinuing.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCanNotJITScriptErrorDebug: OnCanNotJITScriptErrorDebug::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteDebugEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteDebugEx {}
windows_core::imp::define_interface!(IActiveScriptSiteInterruptPoll, IActiveScriptSiteInterruptPoll_Vtbl, 0x539698a0_cdca_11cf_a5eb_00aa0047a063);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteInterruptPoll, windows_core::IUnknown);
impl IActiveScriptSiteInterruptPoll {
    pub unsafe fn QueryContinue(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryContinue)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteInterruptPoll_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteInterruptPoll_Impl: windows_core::IUnknownImpl {
    fn QueryContinue(&self) -> windows_core::Result<()>;
}
impl IActiveScriptSiteInterruptPoll_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryContinue<Identity: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSiteInterruptPoll_Impl::QueryContinue(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryContinue: QueryContinue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteInterruptPoll as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteInterruptPoll {}
windows_core::imp::define_interface!(IActiveScriptSiteTraceInfo, IActiveScriptSiteTraceInfo_Vtbl, 0x4b7272ae_1955_4bfe_98b0_780621888569);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteTraceInfo, windows_core::IUnknown);
impl IActiveScriptSiteTraceInfo {
    pub unsafe fn SendScriptTraceInfo(&self, stieventtype: SCRIPTTRACEINFO, guidcontextid: windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendScriptTraceInfo)(windows_core::Interface::as_raw(self), stieventtype, core::mem::transmute(guidcontextid), dwscriptcontextcookie, lscriptstatementstart, lscriptstatementend, dwreserved).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteTraceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendScriptTraceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPTTRACEINFO, windows_core::GUID, u32, i32, i32, u64) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteTraceInfo_Impl: windows_core::IUnknownImpl {
    fn SendScriptTraceInfo(&self, stieventtype: SCRIPTTRACEINFO, guidcontextid: &windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> windows_core::Result<()>;
}
impl IActiveScriptSiteTraceInfo_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteTraceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendScriptTraceInfo<Identity: IActiveScriptSiteTraceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stieventtype: SCRIPTTRACEINFO, guidcontextid: windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSiteTraceInfo_Impl::SendScriptTraceInfo(this, core::mem::transmute_copy(&stieventtype), core::mem::transmute(&guidcontextid), core::mem::transmute_copy(&dwscriptcontextcookie), core::mem::transmute_copy(&lscriptstatementstart), core::mem::transmute_copy(&lscriptstatementend), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SendScriptTraceInfo: SendScriptTraceInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteTraceInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteTraceInfo {}
windows_core::imp::define_interface!(IActiveScriptSiteUIControl, IActiveScriptSiteUIControl_Vtbl, 0xaedae97e_d7ee_4796_b960_7f092ae844ab);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteUIControl, windows_core::IUnknown);
impl IActiveScriptSiteUIControl {
    pub unsafe fn GetUIBehavior(&self, uicitem: SCRIPTUICITEM) -> windows_core::Result<SCRIPTUICHANDLING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUIBehavior)(windows_core::Interface::as_raw(self), uicitem, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteUIControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUIBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPTUICITEM, *mut SCRIPTUICHANDLING) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteUIControl_Impl: windows_core::IUnknownImpl {
    fn GetUIBehavior(&self, uicitem: SCRIPTUICITEM) -> windows_core::Result<SCRIPTUICHANDLING>;
}
impl IActiveScriptSiteUIControl_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteUIControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUIBehavior<Identity: IActiveScriptSiteUIControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uicitem: SCRIPTUICITEM, puichandling: *mut SCRIPTUICHANDLING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteUIControl_Impl::GetUIBehavior(this, core::mem::transmute_copy(&uicitem)) {
                    Ok(ok__) => {
                        puichandling.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUIBehavior: GetUIBehavior::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteUIControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteUIControl {}
windows_core::imp::define_interface!(IActiveScriptSiteWindow, IActiveScriptSiteWindow_Vtbl, 0xd10f6761_83e9_11cf_8f20_00805f2cd064);
windows_core::imp::interface_hierarchy!(IActiveScriptSiteWindow, windows_core::IUnknown);
impl IActiveScriptSiteWindow {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableModeless(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptSiteWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IActiveScriptSiteWindow_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::super::super::super::Foundation::HWND>;
    fn EnableModeless(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IActiveScriptSiteWindow_Vtbl {
    pub const fn new<Identity: IActiveScriptSiteWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IActiveScriptSiteWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptSiteWindow_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: IActiveScriptSiteWindow_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptSiteWindow_Impl::EnableModeless(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            EnableModeless: EnableModeless::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptSiteWindow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptSiteWindow {}
windows_core::imp::define_interface!(IActiveScriptStats, IActiveScriptStats_Vtbl, 0xb8da6310_e19b_11d0_933c_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IActiveScriptStats, windows_core::IUnknown);
impl IActiveScriptStats {
    pub unsafe fn GetStat(&self, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStat)(windows_core::Interface::as_raw(self), stid, pluhi as _, plulo as _).ok() }
    }
    pub unsafe fn GetStatEx(&self, guid: *const windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatEx)(windows_core::Interface::as_raw(self), guid, pluhi as _, plulo as _).ok() }
    }
    pub unsafe fn ResetStats(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetStats)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptStats_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStatEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub ResetStats: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptStats_Impl: windows_core::IUnknownImpl {
    fn GetStat(&self, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> windows_core::Result<()>;
    fn GetStatEx(&self, guid: *const windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> windows_core::Result<()>;
    fn ResetStats(&self) -> windows_core::Result<()>;
}
impl IActiveScriptStats_Vtbl {
    pub const fn new<Identity: IActiveScriptStats_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStat<Identity: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptStats_Impl::GetStat(this, core::mem::transmute_copy(&stid), core::mem::transmute_copy(&pluhi), core::mem::transmute_copy(&plulo)).into()
            }
        }
        unsafe extern "system" fn GetStatEx<Identity: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptStats_Impl::GetStatEx(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pluhi), core::mem::transmute_copy(&plulo)).into()
            }
        }
        unsafe extern "system" fn ResetStats<Identity: IActiveScriptStats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptStats_Impl::ResetStats(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStat: GetStat::<Identity, OFFSET>,
            GetStatEx: GetStatEx::<Identity, OFFSET>,
            ResetStats: ResetStats::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptStats as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptStats {}
windows_core::imp::define_interface!(IActiveScriptStringCompare, IActiveScriptStringCompare_Vtbl, 0x58562769_ed52_42f7_8403_4963514e1f11);
windows_core::imp::interface_hierarchy!(IActiveScriptStringCompare, windows_core::IUnknown);
impl IActiveScriptStringCompare {
    pub unsafe fn StrComp(&self, bszstr1: &windows_core::BSTR, bszstr2: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrComp)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszstr1), core::mem::transmute_copy(bszstr2), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptStringCompare_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StrComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IActiveScriptStringCompare_Impl: windows_core::IUnknownImpl {
    fn StrComp(&self, bszstr1: &windows_core::BSTR, bszstr2: &windows_core::BSTR) -> windows_core::Result<i32>;
}
impl IActiveScriptStringCompare_Vtbl {
    pub const fn new<Identity: IActiveScriptStringCompare_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StrComp<Identity: IActiveScriptStringCompare_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszstr1: *mut core::ffi::c_void, bszstr2: *mut core::ffi::c_void, iret: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptStringCompare_Impl::StrComp(this, core::mem::transmute(&bszstr1), core::mem::transmute(&bszstr2)) {
                    Ok(ok__) => {
                        iret.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StrComp: StrComp::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptStringCompare as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptStringCompare {}
windows_core::imp::define_interface!(IActiveScriptTraceInfo, IActiveScriptTraceInfo_Vtbl, 0xc35456e7_bebf_4a1b_86a9_24d56be8b369);
windows_core::imp::interface_hierarchy!(IActiveScriptTraceInfo, windows_core::IUnknown);
impl IActiveScriptTraceInfo {
    pub unsafe fn StartScriptTracing<P0>(&self, psitetraceinfo: P0, guidcontextid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptSiteTraceInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartScriptTracing)(windows_core::Interface::as_raw(self), psitetraceinfo.param().abi(), core::mem::transmute(guidcontextid)).ok() }
    }
    pub unsafe fn StopScriptTracing(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopScriptTracing)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptTraceInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartScriptTracing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub StopScriptTracing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveScriptTraceInfo_Impl: windows_core::IUnknownImpl {
    fn StartScriptTracing(&self, psitetraceinfo: windows_core::Ref<'_, IActiveScriptSiteTraceInfo>, guidcontextid: &windows_core::GUID) -> windows_core::Result<()>;
    fn StopScriptTracing(&self) -> windows_core::Result<()>;
}
impl IActiveScriptTraceInfo_Vtbl {
    pub const fn new<Identity: IActiveScriptTraceInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartScriptTracing<Identity: IActiveScriptTraceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psitetraceinfo: *mut core::ffi::c_void, guidcontextid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptTraceInfo_Impl::StartScriptTracing(this, core::mem::transmute_copy(&psitetraceinfo), core::mem::transmute(&guidcontextid)).into()
            }
        }
        unsafe extern "system" fn StopScriptTracing<Identity: IActiveScriptTraceInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveScriptTraceInfo_Impl::StopScriptTracing(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartScriptTracing: StartScriptTracing::<Identity, OFFSET>,
            StopScriptTracing: StopScriptTracing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptTraceInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveScriptTraceInfo {}
windows_core::imp::define_interface!(IActiveScriptWinRTErrorDebug, IActiveScriptWinRTErrorDebug_Vtbl, 0x73a3f82a_0fe9_4b33_ba3b_fe095f697e0a);
impl core::ops::Deref for IActiveScriptWinRTErrorDebug {
    type Target = IActiveScriptError;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveScriptWinRTErrorDebug, windows_core::IUnknown, IActiveScriptError);
impl IActiveScriptWinRTErrorDebug {
    pub unsafe fn GetRestrictedErrorString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictedErrorString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRestrictedErrorReference(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictedErrorReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCapabilitySid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilitySid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveScriptWinRTErrorDebug_Vtbl {
    pub base__: IActiveScriptError_Vtbl,
    pub GetRestrictedErrorString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictedErrorReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilitySid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptWinRTErrorDebug_Impl: IActiveScriptError_Impl {
    fn GetRestrictedErrorString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRestrictedErrorReference(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCapabilitySid(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl IActiveScriptWinRTErrorDebug_Vtbl {
    pub const fn new<Identity: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRestrictedErrorString<Identity: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptWinRTErrorDebug_Impl::GetRestrictedErrorString(this) {
                    Ok(ok__) => {
                        errorstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestrictedErrorReference<Identity: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencestring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptWinRTErrorDebug_Impl::GetRestrictedErrorReference(this) {
                    Ok(ok__) => {
                        referencestring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCapabilitySid<Identity: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilitysid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveScriptWinRTErrorDebug_Impl::GetCapabilitySid(this) {
                    Ok(ok__) => {
                        capabilitysid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IActiveScriptError_Vtbl::new::<Identity, OFFSET>(),
            GetRestrictedErrorString: GetRestrictedErrorString::<Identity, OFFSET>,
            GetRestrictedErrorReference: GetRestrictedErrorReference::<Identity, OFFSET>,
            GetCapabilitySid: GetCapabilitySid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveScriptWinRTErrorDebug as windows_core::Interface>::IID || iid == &<IActiveScriptError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IActiveScriptWinRTErrorDebug {}
windows_core::imp::define_interface!(IApplicationDebugger, IApplicationDebugger_Vtbl, 0x51973c2a_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IApplicationDebugger, windows_core::IUnknown);
impl IApplicationDebugger {
    pub unsafe fn QueryAlive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryAlive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CreateInstanceAtDebugger<P1>(&self, rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceAtDebugger)(windows_core::Interface::as_raw(self), rclsid, punkouter.param().abi(), dwclscontext, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn onDebugOutput<P0>(&self, pstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).onDebugOutput)(windows_core::Interface::as_raw(self), pstr.param().abi()).ok() }
    }
    pub unsafe fn onHandleBreakPoint<P0, P2>(&self, prpt: P0, br: BREAKREASON, perror: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
        P2: windows_core::Param<IActiveScriptErrorDebug>,
    {
        unsafe { (windows_core::Interface::vtable(self).onHandleBreakPoint)(windows_core::Interface::as_raw(self), prpt.param().abi(), br, perror.param().abi()).ok() }
    }
    pub unsafe fn onClose(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onClose)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn onDebuggerEvent<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).onDebuggerEvent)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDebugger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryAlive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceAtDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub onHandleBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BREAKREASON, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onDebuggerEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IApplicationDebugger_Impl: windows_core::IUnknownImpl {
    fn QueryAlive(&self) -> windows_core::Result<()>;
    fn CreateInstanceAtDebugger(&self, rclsid: *const windows_core::GUID, punkouter: windows_core::Ref<'_, windows_core::IUnknown>, dwclscontext: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn onDebugOutput(&self, pstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn onHandleBreakPoint(&self, prpt: windows_core::Ref<'_, IRemoteDebugApplicationThread>, br: BREAKREASON, perror: windows_core::Ref<'_, IActiveScriptErrorDebug>) -> windows_core::Result<()>;
    fn onClose(&self) -> windows_core::Result<()>;
    fn onDebuggerEvent(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IApplicationDebugger_Vtbl {
    pub const fn new<Identity: IApplicationDebugger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryAlive<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebugger_Impl::QueryAlive(this).into()
            }
        }
        unsafe extern "system" fn CreateInstanceAtDebugger<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclscontext: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApplicationDebugger_Impl::CreateInstanceAtDebugger(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppvobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn onDebugOutput<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebugger_Impl::onDebugOutput(this, core::mem::transmute(&pstr)).into()
            }
        }
        unsafe extern "system" fn onHandleBreakPoint<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prpt: *mut core::ffi::c_void, br: BREAKREASON, perror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebugger_Impl::onHandleBreakPoint(this, core::mem::transmute_copy(&prpt), core::mem::transmute_copy(&br), core::mem::transmute_copy(&perror)).into()
            }
        }
        unsafe extern "system" fn onClose<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebugger_Impl::onClose(this).into()
            }
        }
        unsafe extern "system" fn onDebuggerEvent<Identity: IApplicationDebugger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebugger_Impl::onDebuggerEvent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryAlive: QueryAlive::<Identity, OFFSET>,
            CreateInstanceAtDebugger: CreateInstanceAtDebugger::<Identity, OFFSET>,
            onDebugOutput: onDebugOutput::<Identity, OFFSET>,
            onHandleBreakPoint: onHandleBreakPoint::<Identity, OFFSET>,
            onClose: onClose::<Identity, OFFSET>,
            onDebuggerEvent: onDebuggerEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationDebugger as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApplicationDebugger {}
windows_core::imp::define_interface!(IApplicationDebuggerUI, IApplicationDebuggerUI_Vtbl, 0x51973c2b_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IApplicationDebuggerUI, windows_core::IUnknown);
impl IApplicationDebuggerUI {
    pub unsafe fn BringDocumentToTop<P0>(&self, pddt: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentText>,
    {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentToTop)(windows_core::Interface::as_raw(self), pddt.param().abi()).ok() }
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentContextToTop)(windows_core::Interface::as_raw(self), pddc.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDebuggerUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BringDocumentToTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IApplicationDebuggerUI_Impl: windows_core::IUnknownImpl {
    fn BringDocumentToTop(&self, pddt: windows_core::Ref<'_, IDebugDocumentText>) -> windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: windows_core::Ref<'_, IDebugDocumentContext>) -> windows_core::Result<()>;
}
impl IApplicationDebuggerUI_Vtbl {
    pub const fn new<Identity: IApplicationDebuggerUI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BringDocumentToTop<Identity: IApplicationDebuggerUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddt: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebuggerUI_Impl::BringDocumentToTop(this, core::mem::transmute_copy(&pddt)).into()
            }
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: IApplicationDebuggerUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationDebuggerUI_Impl::BringDocumentContextToTop(this, core::mem::transmute_copy(&pddc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BringDocumentToTop: BringDocumentToTop::<Identity, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationDebuggerUI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApplicationDebuggerUI {}
windows_core::imp::define_interface!(IBindEventHandler, IBindEventHandler_Vtbl, 0x63cdbcb0_c1b1_11d0_9336_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IBindEventHandler, windows_core::IUnknown);
impl IBindEventHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindHandler<P0, P1>(&self, pstrevent: P0, pdisp: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).BindHandler)(windows_core::Interface::as_raw(self), pstrevent.param().abi(), pdisp.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BindHandler: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindHandler: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBindEventHandler_Impl: windows_core::IUnknownImpl {
    fn BindHandler(&self, pstrevent: &windows_core::PCWSTR, pdisp: windows_core::Ref<'_, super::super::super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBindEventHandler_Vtbl {
    pub const fn new<Identity: IBindEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindHandler<Identity: IBindEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrevent: windows_core::PCWSTR, pdisp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindEventHandler_Impl::BindHandler(this, core::mem::transmute(&pstrevent), core::mem::transmute_copy(&pdisp)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BindHandler: BindHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBindEventHandler {}
windows_core::imp::define_interface!(IDebugApplication11032, IDebugApplication11032_Vtbl, 0xbdb3b5de_89f2_4e11_84a5_97445f941c7d);
impl core::ops::Deref for IDebugApplication11032 {
    type Target = IRemoteDebugApplication110;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplication11032, windows_core::IUnknown, IRemoteDebugApplication110);
impl IDebugApplication11032 {
    pub unsafe fn SynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall32>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallInMainThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn AsynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall32>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsynchronousCallInMainThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn CallableWaitForHandles(&self, phandles: &[super::super::super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallableWaitForHandles)(windows_core::Interface::as_raw(self), phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication11032_Vtbl {
    pub base__: IRemoteDebugApplication110_Vtbl,
    pub SynchronousCallInMainThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
    pub AsynchronousCallInMainThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
    pub CallableWaitForHandles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
}
pub trait IDebugApplication11032_Impl: IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
    fn AsynchronousCallInMainThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
    fn CallableWaitForHandles(&self, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
}
impl IDebugApplication11032_Vtbl {
    pub const fn new<Identity: IDebugApplication11032_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication11032_Impl::SynchronousCallInMainThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication11032_Impl::AsynchronousCallInMainThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: IDebugApplication11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication11032_Impl::CallableWaitForHandles(this, core::mem::transmute_copy(&handlecount), core::mem::transmute_copy(&phandles)) {
                    Ok(ok__) => {
                        pindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRemoteDebugApplication110_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplication11032 as windows_core::Interface>::IID || iid == &<IRemoteDebugApplication110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplication11032 {}
windows_core::imp::define_interface!(IDebugApplication11064, IDebugApplication11064_Vtbl, 0x2039d958_4eeb_496a_87bb_2e5201eadeef);
impl core::ops::Deref for IDebugApplication11064 {
    type Target = IRemoteDebugApplication110;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplication11064, windows_core::IUnknown, IRemoteDebugApplication110);
impl IDebugApplication11064 {
    pub unsafe fn SynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall64>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallInMainThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn AsynchronousCallInMainThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall64>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsynchronousCallInMainThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn CallableWaitForHandles(&self, phandles: &[super::super::super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CallableWaitForHandles)(windows_core::Interface::as_raw(self), phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication11064_Vtbl {
    pub base__: IRemoteDebugApplication110_Vtbl,
    pub SynchronousCallInMainThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
    pub AsynchronousCallInMainThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
    pub CallableWaitForHandles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
}
pub trait IDebugApplication11064_Impl: IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
    fn AsynchronousCallInMainThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
    fn CallableWaitForHandles(&self, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
}
impl IDebugApplication11064_Vtbl {
    pub const fn new<Identity: IDebugApplication11064_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication11064_Impl::SynchronousCallInMainThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication11064_Impl::AsynchronousCallInMainThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: IDebugApplication11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication11064_Impl::CallableWaitForHandles(this, core::mem::transmute_copy(&handlecount), core::mem::transmute_copy(&phandles)) {
                    Ok(ok__) => {
                        pindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IRemoteDebugApplication110_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplication11064 as windows_core::Interface>::IID || iid == &<IRemoteDebugApplication110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplication11064 {}
windows_core::imp::define_interface!(IDebugApplication32, IDebugApplication32_Vtbl, 0x51973c32_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugApplication32 {
    type Target = IRemoteDebugApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplication32, windows_core::IUnknown, IRemoteDebugApplication);
impl IDebugApplication32 {
    pub unsafe fn SetName<P0>(&self, pstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pstrname.param().abi()).ok() }
    }
    pub unsafe fn StepOutComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StepOutComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DebugOutput<P0>(&self, pstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DebugOutput)(windows_core::Interface::as_raw(self), pstr.param().abi()).ok() }
    }
    pub unsafe fn StartDebugSession(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartDebugSession)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn HandleBreakPoint(&self, br: BREAKREASON) -> windows_core::Result<BREAKRESUMEACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandleBreakPoint)(windows_core::Interface::as_raw(self), br, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut Option<IRemoteDebugApplicationThread>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBreakFlags)(windows_core::Interface::as_raw(self), pabf as _, core::mem::transmute(pprdatsteppingthread)).ok() }
    }
    pub unsafe fn GetCurrentThread(&self) -> windows_core::Result<IDebugApplicationThread> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAsyncDebugOperation<P0>(&self, psdo: P0) -> windows_core::Result<IDebugAsyncOperation>
    where
        P0: windows_core::Param<IDebugSyncOperation>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAsyncDebugOperation)(windows_core::Interface::as_raw(self), psdo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddStackFrameSniffer<P0>(&self, pdsfs: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDebugStackFrameSniffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddStackFrameSniffer)(windows_core::Interface::as_raw(self), pdsfs.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveStackFrameSniffer)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn QueryCurrentThreadIsDebuggerThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryCurrentThreadIsDebuggerThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SynchronousCallInDebuggerThread<P0>(&self, pptc: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall32>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallInDebuggerThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn CreateApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FireDebuggerEvent<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FireDebuggerEvent)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok() }
    }
    pub unsafe fn HandleRuntimeError<P0, P1>(&self, perrordebug: P0, pscriptsite: P1, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptErrorDebug>,
        P1: windows_core::Param<IActiveScriptSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleRuntimeError)(windows_core::Interface::as_raw(self), perrordebug.param().abi(), pscriptsite.param().abi(), pbra as _, perra as _, pfcallonscripterror as _).ok() }
    }
    pub unsafe fn FCanJitDebug(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).FCanJitDebug)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FIsAutoJitDebugEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).FIsAutoJitDebugEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddGlobalExpressionContextProvider<P0>(&self, pdsfs: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IProvideExpressionContexts>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddGlobalExpressionContextProvider)(windows_core::Interface::as_raw(self), pdsfs.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveGlobalExpressionContextProvider)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication32_Vtbl {
    pub base__: IRemoteDebugApplication_Vtbl,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StepOutComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StartDebugSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandleBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, BREAKREASON, *mut BREAKRESUMEACTION) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBreakFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAsyncDebugOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddStackFrameSniffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveStackFrameSniffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryCurrentThreadIsDebuggerThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SynchronousCallInDebuggerThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub CreateApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDebuggerEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandleRuntimeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BREAKRESUMEACTION, *mut ERRORRESUMEACTION, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FCanJitDebug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub FIsAutoJitDebugEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub AddGlobalExpressionContextProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveGlobalExpressionContextProvider: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDebugApplication32_Impl: IRemoteDebugApplication_Impl {
    fn SetName(&self, pstrname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StepOutComplete(&self) -> windows_core::Result<()>;
    fn DebugOutput(&self, pstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StartDebugSession(&self) -> windows_core::Result<()>;
    fn HandleBreakPoint(&self, br: BREAKREASON) -> windows_core::Result<BREAKRESUMEACTION>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: windows_core::OutRef<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn GetCurrentThread(&self) -> windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(&self, psdo: windows_core::Ref<'_, IDebugSyncOperation>) -> windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(&self, pdsfs: windows_core::Ref<'_, IDebugStackFrameSniffer>) -> windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(&self) -> windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()>;
    fn CreateApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn HandleRuntimeError(&self, perrordebug: windows_core::Ref<'_, IActiveScriptErrorDebug>, pscriptsite: windows_core::Ref<'_, IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FCanJitDebug(&self) -> windows_core::BOOL;
    fn FIsAutoJitDebugEnabled(&self) -> windows_core::BOOL;
    fn AddGlobalExpressionContextProvider(&self, pdsfs: windows_core::Ref<'_, IProvideExpressionContexts>) -> windows_core::Result<u32>;
    fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u32) -> windows_core::Result<()>;
}
impl IDebugApplication32_Vtbl {
    pub const fn new<Identity: IDebugApplication32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetName<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::SetName(this, core::mem::transmute(&pstrname)).into()
            }
        }
        unsafe extern "system" fn StepOutComplete<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::StepOutComplete(this).into()
            }
        }
        unsafe extern "system" fn DebugOutput<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::DebugOutput(this, core::mem::transmute(&pstr)).into()
            }
        }
        unsafe extern "system" fn StartDebugSession<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::StartDebugSession(this).into()
            }
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::HandleBreakPoint(this, core::mem::transmute_copy(&br)) {
                    Ok(ok__) => {
                        pbra.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetBreakFlags<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::GetBreakFlags(this, core::mem::transmute_copy(&pabf), core::mem::transmute_copy(&pprdatsteppingthread)).into()
            }
        }
        unsafe extern "system" fn GetCurrentThread<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::GetCurrentThread(this) {
                    Ok(ok__) => {
                        pat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psdo: *mut core::ffi::c_void, ppado: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::CreateAsyncDebugOperation(this, core::mem::transmute_copy(&psdo)) {
                    Ok(ok__) => {
                        ppado.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfs: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::AddStackFrameSniffer(this, core::mem::transmute_copy(&pdsfs)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::RemoveStackFrameSniffer(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::QueryCurrentThreadIsDebuggerThread(this).into()
            }
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::SynchronousCallInDebuggerThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdannew: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::CreateApplicationNode(this) {
                    Ok(ok__) => {
                        ppdannew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::FireDebuggerEvent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrordebug: *mut core::ffi::c_void, pscriptsite: *mut core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::HandleRuntimeError(this, core::mem::transmute_copy(&perrordebug), core::mem::transmute_copy(&pscriptsite), core::mem::transmute_copy(&pbra), core::mem::transmute_copy(&perra), core::mem::transmute_copy(&pfcallonscripterror)).into()
            }
        }
        unsafe extern "system" fn FCanJitDebug<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::FCanJitDebug(this)
            }
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::FIsAutoJitDebugEnabled(this)
            }
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfs: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication32_Impl::AddGlobalExpressionContextProvider(this, core::mem::transmute_copy(&pdsfs)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: IDebugApplication32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication32_Impl::RemoveGlobalExpressionContextProvider(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: IRemoteDebugApplication_Vtbl::new::<Identity, OFFSET>(),
            SetName: SetName::<Identity, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, OFFSET>,
            DebugOutput: DebugOutput::<Identity, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplication32 as windows_core::Interface>::IID || iid == &<IRemoteDebugApplication as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplication32 {}
windows_core::imp::define_interface!(IDebugApplication64, IDebugApplication64_Vtbl, 0x4dedc754_04c7_4f10_9e60_16a390fe6e62);
impl core::ops::Deref for IDebugApplication64 {
    type Target = IRemoteDebugApplication;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplication64, windows_core::IUnknown, IRemoteDebugApplication);
impl IDebugApplication64 {
    pub unsafe fn SetName<P0>(&self, pstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pstrname.param().abi()).ok() }
    }
    pub unsafe fn StepOutComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StepOutComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DebugOutput<P0>(&self, pstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DebugOutput)(windows_core::Interface::as_raw(self), pstr.param().abi()).ok() }
    }
    pub unsafe fn StartDebugSession(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartDebugSession)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn HandleBreakPoint(&self, br: BREAKREASON) -> windows_core::Result<BREAKRESUMEACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandleBreakPoint)(windows_core::Interface::as_raw(self), br, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: *mut Option<IRemoteDebugApplicationThread>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBreakFlags)(windows_core::Interface::as_raw(self), pabf as _, core::mem::transmute(pprdatsteppingthread)).ok() }
    }
    pub unsafe fn GetCurrentThread(&self) -> windows_core::Result<IDebugApplicationThread> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAsyncDebugOperation<P0>(&self, psdo: P0) -> windows_core::Result<IDebugAsyncOperation>
    where
        P0: windows_core::Param<IDebugSyncOperation>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAsyncDebugOperation)(windows_core::Interface::as_raw(self), psdo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddStackFrameSniffer<P0>(&self, pdsfs: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDebugStackFrameSniffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddStackFrameSniffer)(windows_core::Interface::as_raw(self), pdsfs.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveStackFrameSniffer)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn QueryCurrentThreadIsDebuggerThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryCurrentThreadIsDebuggerThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SynchronousCallInDebuggerThread<P0>(&self, pptc: P0, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall64>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallInDebuggerThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn CreateApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FireDebuggerEvent<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FireDebuggerEvent)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok() }
    }
    pub unsafe fn HandleRuntimeError<P0, P1>(&self, perrordebug: P0, pscriptsite: P1, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActiveScriptErrorDebug>,
        P1: windows_core::Param<IActiveScriptSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleRuntimeError)(windows_core::Interface::as_raw(self), perrordebug.param().abi(), pscriptsite.param().abi(), pbra as _, perra as _, pfcallonscripterror as _).ok() }
    }
    pub unsafe fn FCanJitDebug(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).FCanJitDebug)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FIsAutoJitDebugEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).FIsAutoJitDebugEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddGlobalExpressionContextProvider<P0>(&self, pdsfs: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IProvideExpressionContexts>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddGlobalExpressionContextProvider)(windows_core::Interface::as_raw(self), pdsfs.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveGlobalExpressionContextProvider)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplication64_Vtbl {
    pub base__: IRemoteDebugApplication_Vtbl,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StepOutComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub StartDebugSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandleBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, BREAKREASON, *mut BREAKRESUMEACTION) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBreakFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAsyncDebugOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddStackFrameSniffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveStackFrameSniffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryCurrentThreadIsDebuggerThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SynchronousCallInDebuggerThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64, u64) -> windows_core::HRESULT,
    pub CreateApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDebuggerEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandleRuntimeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut BREAKRESUMEACTION, *mut ERRORRESUMEACTION, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub FCanJitDebug: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub FIsAutoJitDebugEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub AddGlobalExpressionContextProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub RemoveGlobalExpressionContextProvider: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IDebugApplication64_Impl: IRemoteDebugApplication_Impl {
    fn SetName(&self, pstrname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StepOutComplete(&self) -> windows_core::Result<()>;
    fn DebugOutput(&self, pstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StartDebugSession(&self) -> windows_core::Result<()>;
    fn HandleBreakPoint(&self, br: BREAKREASON) -> windows_core::Result<BREAKRESUMEACTION>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetBreakFlags(&self, pabf: *mut u32, pprdatsteppingthread: windows_core::OutRef<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn GetCurrentThread(&self) -> windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(&self, psdo: windows_core::Ref<'_, IDebugSyncOperation>) -> windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(&self, pdsfs: windows_core::Ref<'_, IDebugStackFrameSniffer>) -> windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(&self) -> windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()>;
    fn CreateApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
    fn HandleRuntimeError(&self, perrordebug: windows_core::Ref<'_, IActiveScriptErrorDebug>, pscriptsite: windows_core::Ref<'_, IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn FCanJitDebug(&self) -> windows_core::BOOL;
    fn FIsAutoJitDebugEnabled(&self) -> windows_core::BOOL;
    fn AddGlobalExpressionContextProvider(&self, pdsfs: windows_core::Ref<'_, IProvideExpressionContexts>) -> windows_core::Result<u64>;
    fn RemoveGlobalExpressionContextProvider(&self, dwcookie: u64) -> windows_core::Result<()>;
}
impl IDebugApplication64_Vtbl {
    pub const fn new<Identity: IDebugApplication64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetName<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::SetName(this, core::mem::transmute(&pstrname)).into()
            }
        }
        unsafe extern "system" fn StepOutComplete<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::StepOutComplete(this).into()
            }
        }
        unsafe extern "system" fn DebugOutput<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::DebugOutput(this, core::mem::transmute(&pstr)).into()
            }
        }
        unsafe extern "system" fn StartDebugSession<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::StartDebugSession(this).into()
            }
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::HandleBreakPoint(this, core::mem::transmute_copy(&br)) {
                    Ok(ok__) => {
                        pbra.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetBreakFlags<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::GetBreakFlags(this, core::mem::transmute_copy(&pabf), core::mem::transmute_copy(&pprdatsteppingthread)).into()
            }
        }
        unsafe extern "system" fn GetCurrentThread<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::GetCurrentThread(this) {
                    Ok(ok__) => {
                        pat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psdo: *mut core::ffi::c_void, ppado: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::CreateAsyncDebugOperation(this, core::mem::transmute_copy(&psdo)) {
                    Ok(ok__) => {
                        ppado.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfs: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::AddStackFrameSniffer(this, core::mem::transmute_copy(&pdsfs)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::RemoveStackFrameSniffer(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::QueryCurrentThreadIsDebuggerThread(this).into()
            }
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::SynchronousCallInDebuggerThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdannew: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::CreateApplicationNode(this) {
                    Ok(ok__) => {
                        ppdannew.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::FireDebuggerEvent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrordebug: *mut core::ffi::c_void, pscriptsite: *mut core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::HandleRuntimeError(this, core::mem::transmute_copy(&perrordebug), core::mem::transmute_copy(&pscriptsite), core::mem::transmute_copy(&pbra), core::mem::transmute_copy(&perra), core::mem::transmute_copy(&pfcallonscripterror)).into()
            }
        }
        unsafe extern "system" fn FCanJitDebug<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::FCanJitDebug(this)
            }
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::FIsAutoJitDebugEnabled(this)
            }
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfs: *mut core::ffi::c_void, pdwcookie: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplication64_Impl::AddGlobalExpressionContextProvider(this, core::mem::transmute_copy(&pdsfs)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: IDebugApplication64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplication64_Impl::RemoveGlobalExpressionContextProvider(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: IRemoteDebugApplication_Vtbl::new::<Identity, OFFSET>(),
            SetName: SetName::<Identity, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, OFFSET>,
            DebugOutput: DebugOutput::<Identity, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplication64 as windows_core::Interface>::IID || iid == &<IRemoteDebugApplication as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplication64 {}
windows_core::imp::define_interface!(IDebugApplicationNode, IDebugApplicationNode_Vtbl, 0x51973c34_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugApplicationNode {
    type Target = IDebugDocumentProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplicationNode, windows_core::IUnknown, IDebugDocumentInfo, IDebugDocumentProvider);
impl IDebugApplicationNode {
    pub unsafe fn EnumChildren(&self) -> windows_core::Result<IEnumDebugApplicationNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDocumentProvider<P0>(&self, pddp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentProvider)(windows_core::Interface::as_raw(self), pddp.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Attach<P0>(&self, pdanparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), pdanparent.param().abi()).ok() }
    }
    pub unsafe fn Detach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNode_Vtbl {
    pub base__: IDebugDocumentProvider_Vtbl,
    pub EnumChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDocumentProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugApplicationNode_Impl: IDebugDocumentProvider_Impl {
    fn EnumChildren(&self) -> windows_core::Result<IEnumDebugApplicationNodes>;
    fn GetParent(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn SetDocumentProvider(&self, pddp: windows_core::Ref<'_, IDebugDocumentProvider>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Attach(&self, pdanparent: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
}
impl IDebugApplicationNode_Vtbl {
    pub const fn new<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumChildren<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperddp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationNode_Impl::EnumChildren(this) {
                    Ok(ok__) => {
                        pperddp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprddp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationNode_Impl::GetParent(this) {
                    Ok(ok__) => {
                        pprddp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDocumentProvider<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode_Impl::SetDocumentProvider(this, core::mem::transmute_copy(&pddp)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Attach<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdanparent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode_Impl::Attach(this, core::mem::transmute_copy(&pdanparent)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IDebugApplicationNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode_Impl::Detach(this).into()
            }
        }
        Self {
            base__: IDebugDocumentProvider_Vtbl::new::<Identity, OFFSET>(),
            EnumChildren: EnumChildren::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
            SetDocumentProvider: SetDocumentProvider::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationNode as windows_core::Interface>::IID || iid == &<IDebugDocumentInfo as windows_core::Interface>::IID || iid == &<IDebugDocumentProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationNode {}
windows_core::imp::define_interface!(IDebugApplicationNode100, IDebugApplicationNode100_Vtbl, 0x90a7734e_841b_4f77_9384_a2891e76e7e2);
windows_core::imp::interface_hierarchy!(IDebugApplicationNode100, windows_core::IUnknown);
impl IDebugApplicationNode100 {
    pub unsafe fn SetFilterForEventSink(&self, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilterForEventSink)(windows_core::Interface::as_raw(self), dwcookie, filter).ok() }
    }
    pub unsafe fn GetExcludedDocuments(&self, filter: APPLICATION_NODE_EVENT_FILTER) -> windows_core::Result<TEXT_DOCUMENT_ARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludedDocuments)(windows_core::Interface::as_raw(self), filter, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryIsChildNode<P0>(&self, psearchkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocument>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryIsChildNode)(windows_core::Interface::as_raw(self), psearchkey.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNode100_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFilterForEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, APPLICATION_NODE_EVENT_FILTER) -> windows_core::HRESULT,
    pub GetExcludedDocuments: unsafe extern "system" fn(*mut core::ffi::c_void, APPLICATION_NODE_EVENT_FILTER, *mut TEXT_DOCUMENT_ARRAY) -> windows_core::HRESULT,
    pub QueryIsChildNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugApplicationNode100_Impl: windows_core::IUnknownImpl {
    fn SetFilterForEventSink(&self, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> windows_core::Result<()>;
    fn GetExcludedDocuments(&self, filter: APPLICATION_NODE_EVENT_FILTER) -> windows_core::Result<TEXT_DOCUMENT_ARRAY>;
    fn QueryIsChildNode(&self, psearchkey: windows_core::Ref<'_, IDebugDocument>) -> windows_core::Result<()>;
}
impl IDebugApplicationNode100_Vtbl {
    pub const fn new<Identity: IDebugApplicationNode100_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFilterForEventSink<Identity: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode100_Impl::SetFilterForEventSink(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn GetExcludedDocuments<Identity: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: APPLICATION_NODE_EVENT_FILTER, pdocuments: *mut TEXT_DOCUMENT_ARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationNode100_Impl::GetExcludedDocuments(this, core::mem::transmute_copy(&filter)) {
                    Ok(ok__) => {
                        pdocuments.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryIsChildNode<Identity: IDebugApplicationNode100_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNode100_Impl::QueryIsChildNode(this, core::mem::transmute_copy(&psearchkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFilterForEventSink: SetFilterForEventSink::<Identity, OFFSET>,
            GetExcludedDocuments: GetExcludedDocuments::<Identity, OFFSET>,
            QueryIsChildNode: QueryIsChildNode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationNode100 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationNode100 {}
windows_core::imp::define_interface!(IDebugApplicationNodeEvents, IDebugApplicationNodeEvents_Vtbl, 0x51973c35_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugApplicationNodeEvents, windows_core::IUnknown);
impl IDebugApplicationNodeEvents {
    pub unsafe fn onAddChild<P0>(&self, prddpchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).onAddChild)(windows_core::Interface::as_raw(self), prddpchild.param().abi()).ok() }
    }
    pub unsafe fn onRemoveChild<P0>(&self, prddpchild: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).onRemoveChild)(windows_core::Interface::as_raw(self), prddpchild.param().abi()).ok() }
    }
    pub unsafe fn onDetach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onDetach)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn onAttach<P0>(&self, prddpparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplicationNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).onAttach)(windows_core::Interface::as_raw(self), prddpparent.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationNodeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub onAddChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onRemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onDetach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugApplicationNodeEvents_Impl: windows_core::IUnknownImpl {
    fn onAddChild(&self, prddpchild: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn onRemoveChild(&self, prddpchild: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
    fn onDetach(&self) -> windows_core::Result<()>;
    fn onAttach(&self, prddpparent: windows_core::Ref<'_, IDebugApplicationNode>) -> windows_core::Result<()>;
}
impl IDebugApplicationNodeEvents_Vtbl {
    pub const fn new<Identity: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn onAddChild<Identity: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNodeEvents_Impl::onAddChild(this, core::mem::transmute_copy(&prddpchild)).into()
            }
        }
        unsafe extern "system" fn onRemoveChild<Identity: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNodeEvents_Impl::onRemoveChild(this, core::mem::transmute_copy(&prddpchild)).into()
            }
        }
        unsafe extern "system" fn onDetach<Identity: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNodeEvents_Impl::onDetach(this).into()
            }
        }
        unsafe extern "system" fn onAttach<Identity: IDebugApplicationNodeEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prddpparent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationNodeEvents_Impl::onAttach(this, core::mem::transmute_copy(&prddpparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onAddChild: onAddChild::<Identity, OFFSET>,
            onRemoveChild: onRemoveChild::<Identity, OFFSET>,
            onDetach: onDetach::<Identity, OFFSET>,
            onAttach: onAttach::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationNodeEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationNodeEvents {}
windows_core::imp::define_interface!(IDebugApplicationThread, IDebugApplicationThread_Vtbl, 0x51973c38_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugApplicationThread {
    type Target = IRemoteDebugApplicationThread;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplicationThread, windows_core::IUnknown, IRemoteDebugApplicationThread);
impl IDebugApplicationThread {
    pub unsafe fn SynchronousCallIntoThread32<P0>(&self, pstcb: P0, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall32>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallIntoThread32)(windows_core::Interface::as_raw(self), pstcb.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
    pub unsafe fn QueryIsCurrentThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryIsCurrentThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn QueryIsDebuggerThread(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryIsDebuggerThread)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetDescription<P0>(&self, pstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), pstrdescription.param().abi()).ok() }
    }
    pub unsafe fn SetStateString<P0>(&self, pstrstate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStateString)(windows_core::Interface::as_raw(self), pstrstate.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread_Vtbl {
    pub base__: IRemoteDebugApplicationThread_Vtbl,
    pub SynchronousCallIntoThread32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub QueryIsCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryIsDebuggerThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetStateString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDebugApplicationThread_Impl: IRemoteDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread32(&self, pstcb: windows_core::Ref<'_, IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()>;
    fn QueryIsCurrentThread(&self) -> windows_core::Result<()>;
    fn QueryIsDebuggerThread(&self) -> windows_core::Result<()>;
    fn SetDescription(&self, pstrdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetStateString(&self, pstrstate: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDebugApplicationThread_Vtbl {
    pub const fn new<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCallIntoThread32<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstcb: *mut core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread_Impl::SynchronousCallIntoThread32(this, core::mem::transmute_copy(&pstcb), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        unsafe extern "system" fn QueryIsCurrentThread<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread_Impl::QueryIsCurrentThread(this).into()
            }
        }
        unsafe extern "system" fn QueryIsDebuggerThread<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread_Impl::QueryIsDebuggerThread(this).into()
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread_Impl::SetDescription(this, core::mem::transmute(&pstrdescription)).into()
            }
        }
        unsafe extern "system" fn SetStateString<Identity: IDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrstate: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread_Impl::SetStateString(this, core::mem::transmute(&pstrstate)).into()
            }
        }
        Self {
            base__: IRemoteDebugApplicationThread_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCallIntoThread32: SynchronousCallIntoThread32::<Identity, OFFSET>,
            QueryIsCurrentThread: QueryIsCurrentThread::<Identity, OFFSET>,
            QueryIsDebuggerThread: QueryIsDebuggerThread::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            SetStateString: SetStateString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationThread as windows_core::Interface>::IID || iid == &<IRemoteDebugApplicationThread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationThread {}
windows_core::imp::define_interface!(IDebugApplicationThread11032, IDebugApplicationThread11032_Vtbl, 0x2194ac5c_6561_404a_a2e9_f57d72de3702);
windows_core::imp::interface_hierarchy!(IDebugApplicationThread11032, windows_core::IUnknown);
impl IDebugApplicationThread11032 {
    pub unsafe fn GetActiveThreadRequestCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveThreadRequestCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSuspendedForBreakPoint(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSuspendedForBreakPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsThreadCallable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThreadCallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AsynchronousCallIntoThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall32>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsynchronousCallIntoThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread11032_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActiveThreadRequestCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsSuspendedForBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsThreadCallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AsynchronousCallIntoThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
}
pub trait IDebugApplicationThread11032_Impl: windows_core::IUnknownImpl {
    fn GetActiveThreadRequestCount(&self) -> windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsThreadCallable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AsynchronousCallIntoThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
}
impl IDebugApplicationThread11032_Vtbl {
    pub const fn new<Identity: IDebugApplicationThread11032_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puithreadrequests: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11032_Impl::GetActiveThreadRequestCount(this) {
                    Ok(ok__) => {
                        puithreadrequests.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfissuspended: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11032_Impl::IsSuspendedForBreakPoint(this) {
                    Ok(ok__) => {
                        pfissuspended.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsThreadCallable<Identity: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscallable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11032_Impl::IsThreadCallable(this) {
                    Ok(ok__) => {
                        pfiscallable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: IDebugApplicationThread11032_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread11032_Impl::AsynchronousCallIntoThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationThread11032 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationThread11032 {}
windows_core::imp::define_interface!(IDebugApplicationThread11064, IDebugApplicationThread11064_Vtbl, 0x420aa4cc_efd8_4dac_983b_47127826917d);
windows_core::imp::interface_hierarchy!(IDebugApplicationThread11064, windows_core::IUnknown);
impl IDebugApplicationThread11064 {
    pub unsafe fn GetActiveThreadRequestCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveThreadRequestCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSuspendedForBreakPoint(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSuspendedForBreakPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsThreadCallable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThreadCallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AsynchronousCallIntoThread<P0>(&self, pptc: P0, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall64>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsynchronousCallIntoThread)(windows_core::Interface::as_raw(self), pptc.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread11064_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActiveThreadRequestCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsSuspendedForBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsThreadCallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AsynchronousCallIntoThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize, usize, usize) -> windows_core::HRESULT,
}
pub trait IDebugApplicationThread11064_Impl: windows_core::IUnknownImpl {
    fn GetActiveThreadRequestCount(&self) -> windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsThreadCallable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AsynchronousCallIntoThread(&self, pptc: windows_core::Ref<'_, IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::Result<()>;
}
impl IDebugApplicationThread11064_Vtbl {
    pub const fn new<Identity: IDebugApplicationThread11064_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puithreadrequests: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11064_Impl::GetActiveThreadRequestCount(this) {
                    Ok(ok__) => {
                        puithreadrequests.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfissuspended: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11064_Impl::IsSuspendedForBreakPoint(this) {
                    Ok(ok__) => {
                        pfissuspended.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsThreadCallable<Identity: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscallable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugApplicationThread11064_Impl::IsThreadCallable(this) {
                    Ok(ok__) => {
                        pfiscallable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: IDebugApplicationThread11064_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptc: *mut core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread11064_Impl::AsynchronousCallIntoThread(this, core::mem::transmute_copy(&pptc), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationThread11064 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationThread11064 {}
windows_core::imp::define_interface!(IDebugApplicationThread64, IDebugApplicationThread64_Vtbl, 0x9dac5886_dbad_456d_9dee_5dec39ab3dda);
impl core::ops::Deref for IDebugApplicationThread64 {
    type Target = IDebugApplicationThread;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugApplicationThread64, windows_core::IUnknown, IRemoteDebugApplicationThread, IDebugApplicationThread);
impl IDebugApplicationThread64 {
    pub unsafe fn SynchronousCallIntoThread64<P0>(&self, pstcb: P0, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugThreadCall64>,
    {
        unsafe { (windows_core::Interface::vtable(self).SynchronousCallIntoThread64)(windows_core::Interface::as_raw(self), pstcb.param().abi(), dwparam1, dwparam2, dwparam3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThread64_Vtbl {
    pub base__: IDebugApplicationThread_Vtbl,
    pub SynchronousCallIntoThread64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64, u64) -> windows_core::HRESULT,
}
pub trait IDebugApplicationThread64_Impl: IDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread64(&self, pstcb: windows_core::Ref<'_, IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()>;
}
impl IDebugApplicationThread64_Vtbl {
    pub const fn new<Identity: IDebugApplicationThread64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SynchronousCallIntoThread64<Identity: IDebugApplicationThread64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstcb: *mut core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThread64_Impl::SynchronousCallIntoThread64(this, core::mem::transmute_copy(&pstcb), core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        Self { base__: IDebugApplicationThread_Vtbl::new::<Identity, OFFSET>(), SynchronousCallIntoThread64: SynchronousCallIntoThread64::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationThread64 as windows_core::Interface>::IID || iid == &<IRemoteDebugApplicationThread as windows_core::Interface>::IID || iid == &<IDebugApplicationThread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationThread64 {}
windows_core::imp::define_interface!(IDebugApplicationThreadEvents110, IDebugApplicationThreadEvents110_Vtbl, 0x84e5e468_d5da_48a8_83f4_40366429007b);
windows_core::imp::interface_hierarchy!(IDebugApplicationThreadEvents110, windows_core::IUnknown);
impl IDebugApplicationThreadEvents110 {
    pub unsafe fn OnSuspendForBreakPoint(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSuspendForBreakPoint)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnResumeFromBreakPoint(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnResumeFromBreakPoint)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnThreadRequestComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnThreadRequestComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnBeginThreadRequest(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBeginThreadRequest)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugApplicationThreadEvents110_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSuspendForBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnResumeFromBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnThreadRequestComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBeginThreadRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugApplicationThreadEvents110_Impl: windows_core::IUnknownImpl {
    fn OnSuspendForBreakPoint(&self) -> windows_core::Result<()>;
    fn OnResumeFromBreakPoint(&self) -> windows_core::Result<()>;
    fn OnThreadRequestComplete(&self) -> windows_core::Result<()>;
    fn OnBeginThreadRequest(&self) -> windows_core::Result<()>;
}
impl IDebugApplicationThreadEvents110_Vtbl {
    pub const fn new<Identity: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSuspendForBreakPoint<Identity: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThreadEvents110_Impl::OnSuspendForBreakPoint(this).into()
            }
        }
        unsafe extern "system" fn OnResumeFromBreakPoint<Identity: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThreadEvents110_Impl::OnResumeFromBreakPoint(this).into()
            }
        }
        unsafe extern "system" fn OnThreadRequestComplete<Identity: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThreadEvents110_Impl::OnThreadRequestComplete(this).into()
            }
        }
        unsafe extern "system" fn OnBeginThreadRequest<Identity: IDebugApplicationThreadEvents110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugApplicationThreadEvents110_Impl::OnBeginThreadRequest(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSuspendForBreakPoint: OnSuspendForBreakPoint::<Identity, OFFSET>,
            OnResumeFromBreakPoint: OnResumeFromBreakPoint::<Identity, OFFSET>,
            OnThreadRequestComplete: OnThreadRequestComplete::<Identity, OFFSET>,
            OnBeginThreadRequest: OnBeginThreadRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugApplicationThreadEvents110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugApplicationThreadEvents110 {}
windows_core::imp::define_interface!(IDebugAsyncOperation, IDebugAsyncOperation_Vtbl, 0x51973c1b_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugAsyncOperation, windows_core::IUnknown);
impl IDebugAsyncOperation {
    pub unsafe fn GetSyncDebugOperation(&self) -> windows_core::Result<IDebugSyncOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSyncDebugOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Start<P0>(&self, padocb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugAsyncOperationCallBack>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), padocb.param().abi()).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn QueryIsComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryIsComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetResult(&self, phrresult: *mut windows_core::HRESULT, ppunkresult: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResult)(windows_core::Interface::as_raw(self), phrresult as _, core::mem::transmute(ppunkresult)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugAsyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSyncDebugOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryIsComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugAsyncOperation_Impl: windows_core::IUnknownImpl {
    fn GetSyncDebugOperation(&self) -> windows_core::Result<IDebugSyncOperation>;
    fn Start(&self, padocb: windows_core::Ref<'_, IDebugAsyncOperationCallBack>) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn QueryIsComplete(&self) -> windows_core::Result<()>;
    fn GetResult(&self, phrresult: *mut windows_core::HRESULT, ppunkresult: windows_core::OutRef<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IDebugAsyncOperation_Vtbl {
    pub const fn new<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSyncDebugOperation<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugAsyncOperation_Impl::GetSyncDebugOperation(this) {
                    Ok(ok__) => {
                        ppsdo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padocb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugAsyncOperation_Impl::Start(this, core::mem::transmute_copy(&padocb)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugAsyncOperation_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn QueryIsComplete<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugAsyncOperation_Impl::QueryIsComplete(this).into()
            }
        }
        unsafe extern "system" fn GetResult<Identity: IDebugAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, ppunkresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugAsyncOperation_Impl::GetResult(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&ppunkresult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSyncDebugOperation: GetSyncDebugOperation::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugAsyncOperation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugAsyncOperation {}
windows_core::imp::define_interface!(IDebugAsyncOperationCallBack, IDebugAsyncOperationCallBack_Vtbl, 0x51973c1c_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugAsyncOperationCallBack, windows_core::IUnknown);
impl IDebugAsyncOperationCallBack {
    pub unsafe fn onComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugAsyncOperationCallBack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugAsyncOperationCallBack_Impl: windows_core::IUnknownImpl {
    fn onComplete(&self) -> windows_core::Result<()>;
}
impl IDebugAsyncOperationCallBack_Vtbl {
    pub const fn new<Identity: IDebugAsyncOperationCallBack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn onComplete<Identity: IDebugAsyncOperationCallBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugAsyncOperationCallBack_Impl::onComplete(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugAsyncOperationCallBack as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugAsyncOperationCallBack {}
windows_core::imp::define_interface!(IDebugCodeContext, IDebugCodeContext_Vtbl, 0x51973c13_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugCodeContext, windows_core::IUnknown);
impl IDebugCodeContext {
    pub unsafe fn GetDocumentContext(&self) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBreakPoint(&self, bps: BREAKPOINT_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakPoint)(windows_core::Interface::as_raw(self), bps).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugCodeContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocumentContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, BREAKPOINT_STATE) -> windows_core::HRESULT,
}
pub trait IDebugCodeContext_Impl: windows_core::IUnknownImpl {
    fn GetDocumentContext(&self) -> windows_core::Result<IDebugDocumentContext>;
    fn SetBreakPoint(&self, bps: BREAKPOINT_STATE) -> windows_core::Result<()>;
}
impl IDebugCodeContext_Vtbl {
    pub const fn new<Identity: IDebugCodeContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentContext<Identity: IDebugCodeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugCodeContext_Impl::GetDocumentContext(this) {
                    Ok(ok__) => {
                        ppsc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBreakPoint<Identity: IDebugCodeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bps: BREAKPOINT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugCodeContext_Impl::SetBreakPoint(this, core::mem::transmute_copy(&bps)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentContext: GetDocumentContext::<Identity, OFFSET>,
            SetBreakPoint: SetBreakPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugCodeContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugCodeContext {}
windows_core::imp::define_interface!(IDebugCookie, IDebugCookie_Vtbl, 0x51973c39_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugCookie, windows_core::IUnknown);
impl IDebugCookie {
    pub unsafe fn SetDebugCookie(&self, dwdebugappcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDebugCookie)(windows_core::Interface::as_raw(self), dwdebugappcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugCookie_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDebugCookie: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDebugCookie_Impl: windows_core::IUnknownImpl {
    fn SetDebugCookie(&self, dwdebugappcookie: u32) -> windows_core::Result<()>;
}
impl IDebugCookie_Vtbl {
    pub const fn new<Identity: IDebugCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDebugCookie<Identity: IDebugCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdebugappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugCookie_Impl::SetDebugCookie(this, core::mem::transmute_copy(&dwdebugappcookie)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetDebugCookie: SetDebugCookie::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugCookie {}
windows_core::imp::define_interface!(IDebugDocument, IDebugDocument_Vtbl, 0x51973c21_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugDocument {
    type Target = IDebugDocumentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugDocument, windows_core::IUnknown, IDebugDocumentInfo);
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocument_Vtbl {
    pub base__: IDebugDocumentInfo_Vtbl,
}
pub trait IDebugDocument_Impl: IDebugDocumentInfo_Impl {}
impl IDebugDocument_Vtbl {
    pub const fn new<Identity: IDebugDocument_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDebugDocumentInfo_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocument as windows_core::Interface>::IID || iid == &<IDebugDocumentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocument {}
windows_core::imp::define_interface!(IDebugDocumentContext, IDebugDocumentContext_Vtbl, 0x51973c28_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentContext, windows_core::IUnknown);
impl IDebugDocumentContext {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<IDebugDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumCodeContexts(&self) -> windows_core::Result<IEnumDebugCodeContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumCodeContexts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumCodeContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentContext_Impl: windows_core::IUnknownImpl {
    fn GetDocument(&self) -> windows_core::Result<IDebugDocument>;
    fn EnumCodeContexts(&self) -> windows_core::Result<IEnumDebugCodeContexts>;
}
impl IDebugDocumentContext_Vtbl {
    pub const fn new<Identity: IDebugDocumentContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocument<Identity: IDebugDocumentContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentContext_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        ppsd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumCodeContexts<Identity: IDebugDocumentContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppescc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentContext_Impl::EnumCodeContexts(this) {
                    Ok(ok__) => {
                        ppescc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDocument: GetDocument::<Identity, OFFSET>,
            EnumCodeContexts: EnumCodeContexts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentContext {}
windows_core::imp::define_interface!(IDebugDocumentHelper32, IDebugDocumentHelper32_Vtbl, 0x51973c26_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentHelper32, windows_core::IUnknown);
impl IDebugDocumentHelper32 {
    pub unsafe fn Init<P0, P1, P2>(&self, pda: P0, pszshortname: P1, pszlongname: P2, docattr: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplication32>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pda.param().abi(), pszshortname.param().abi(), pszlongname.param().abi(), docattr).ok() }
    }
    pub unsafe fn Attach<P0>(&self, pddhparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentHelper32>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), pddhparent.param().abi()).ok() }
    }
    pub unsafe fn Detach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddUnicodeText<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUnicodeText)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn AddDBCSText<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDBCSText)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn SetDebugDocumentHost<P0>(&self, pddh: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentHost>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDebugDocumentHost)(windows_core::Interface::as_raw(self), pddh.param().abi()).ok() }
    }
    pub unsafe fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDeferredText)(windows_core::Interface::as_raw(self), cchars, dwtextstartcookie).ok() }
    }
    pub unsafe fn DefineScriptBlock<P2>(&self, ulcharoffset: u32, cchars: u32, pas: P2, fscriptlet: bool) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<IActiveScript>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefineScriptBlock)(windows_core::Interface::as_raw(self), ulcharoffset, cchars, pas.param().abi(), fscriptlet.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultTextAttr(&self, statextattr: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultTextAttr)(windows_core::Interface::as_raw(self), statextattr).ok() }
    }
    pub unsafe fn SetTextAttributes(&self, ulcharoffset: u32, pstatextattr: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTextAttributes)(windows_core::Interface::as_raw(self), ulcharoffset, pstatextattr.len().try_into().unwrap(), core::mem::transmute(pstatextattr.as_ptr())).ok() }
    }
    pub unsafe fn SetLongName<P0>(&self, pszlongname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLongName)(windows_core::Interface::as_raw(self), pszlongname.param().abi()).ok() }
    }
    pub unsafe fn SetShortName<P0>(&self, pszshortname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetShortName)(windows_core::Interface::as_raw(self), pszshortname.param().abi()).ok() }
    }
    pub unsafe fn SetDocumentAttr(&self, pszattributes: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentAttr)(windows_core::Interface::as_raw(self), pszattributes).ok() }
    }
    pub unsafe fn GetDebugApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDebugApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetScriptBlockInfo(&self, dwsourcecontext: u32, ppasd: *mut Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScriptBlockInfo)(windows_core::Interface::as_raw(self), dwsourcecontext, core::mem::transmute(ppasd), picharpos as _, pcchars as _).ok() }
    }
    pub unsafe fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDebugDocumentContext)(windows_core::Interface::as_raw(self), icharpos, cchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BringDocumentToTop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentToTop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentContextToTop)(windows_core::Interface::as_raw(self), pddc.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHelper32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddUnicodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddDBCSText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetDebugDocumentHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDeferredText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DefineScriptBlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub SetDefaultTextAttr: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SetTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u16) -> windows_core::HRESULT,
    pub SetLongName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetShortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetDocumentAttr: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScriptBlockInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub CreateDebugDocumentContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BringDocumentToTop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentHelper32_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pda: windows_core::Ref<'_, IDebugApplication32>, pszshortname: &windows_core::PCWSTR, pszlongname: &windows_core::PCWSTR, docattr: u32) -> windows_core::Result<()>;
    fn Attach(&self, pddhparent: windows_core::Ref<'_, IDebugDocumentHelper32>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
    fn AddUnicodeText(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDBCSText(&self, psztext: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetDebugDocumentHost(&self, pddh: windows_core::Ref<'_, IDebugDocumentHost>) -> windows_core::Result<()>;
    fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> windows_core::Result<()>;
    fn DefineScriptBlock(&self, ulcharoffset: u32, cchars: u32, pas: windows_core::Ref<'_, IActiveScript>, fscriptlet: windows_core::BOOL) -> windows_core::Result<u32>;
    fn SetDefaultTextAttr(&self, statextattr: u16) -> windows_core::Result<()>;
    fn SetTextAttributes(&self, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> windows_core::Result<()>;
    fn SetLongName(&self, pszlongname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetShortName(&self, pszshortname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDocumentAttr(&self, pszattributes: u32) -> windows_core::Result<()>;
    fn GetDebugApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(&self, dwsourcecontext: u32, ppasd: windows_core::OutRef<'_, IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::Result<()>;
    fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(&self) -> windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: windows_core::Ref<'_, IDebugDocumentContext>) -> windows_core::Result<()>;
}
impl IDebugDocumentHelper32_Vtbl {
    pub const fn new<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, pszshortname: windows_core::PCWSTR, pszlongname: windows_core::PCWSTR, docattr: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::Init(this, core::mem::transmute_copy(&pda), core::mem::transmute(&pszshortname), core::mem::transmute(&pszlongname), core::mem::transmute_copy(&docattr)).into()
            }
        }
        unsafe extern "system" fn Attach<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddhparent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::Attach(this, core::mem::transmute_copy(&pddhparent)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::Detach(this).into()
            }
        }
        unsafe extern "system" fn AddUnicodeText<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::AddUnicodeText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn AddDBCSText<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::AddDBCSText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddh: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetDebugDocumentHost(this, core::mem::transmute_copy(&pddh)).into()
            }
        }
        unsafe extern "system" fn AddDeferredText<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::AddDeferredText(this, core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&dwtextstartcookie)).into()
            }
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut core::ffi::c_void, fscriptlet: windows_core::BOOL, pdwsourcecontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper32_Impl::DefineScriptBlock(this, core::mem::transmute_copy(&ulcharoffset), core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&pas), core::mem::transmute_copy(&fscriptlet)) {
                    Ok(ok__) => {
                        pdwsourcecontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statextattr: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetDefaultTextAttr(this, core::mem::transmute_copy(&statextattr)).into()
            }
        }
        unsafe extern "system" fn SetTextAttributes<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetTextAttributes(this, core::mem::transmute_copy(&ulcharoffset), core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&pstatextattr)).into()
            }
        }
        unsafe extern "system" fn SetLongName<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlongname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetLongName(this, core::mem::transmute(&pszlongname)).into()
            }
        }
        unsafe extern "system" fn SetShortName<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszshortname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetShortName(this, core::mem::transmute(&pszshortname)).into()
            }
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::SetDocumentAttr(this, core::mem::transmute_copy(&pszattributes)).into()
            }
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdan: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper32_Impl::GetDebugApplicationNode(this) {
                    Ok(ok__) => {
                        ppdan.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u32, ppasd: *mut *mut core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::GetScriptBlockInfo(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ppasd), core::mem::transmute_copy(&picharpos), core::mem::transmute_copy(&pcchars)).into()
            }
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper32_Impl::CreateDebugDocumentContext(this, core::mem::transmute_copy(&icharpos), core::mem::transmute_copy(&cchars)) {
                    Ok(ok__) => {
                        ppddc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::BringDocumentToTop(this).into()
            }
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: IDebugDocumentHelper32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper32_Impl::BringDocumentContextToTop(this, core::mem::transmute_copy(&pddc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, OFFSET>,
            SetLongName: SetLongName::<Identity, OFFSET>,
            SetShortName: SetShortName::<Identity, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentHelper32 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentHelper32 {}
windows_core::imp::define_interface!(IDebugDocumentHelper64, IDebugDocumentHelper64_Vtbl, 0xc4c7363c_20fd_47f9_bd82_4855e0150871);
windows_core::imp::interface_hierarchy!(IDebugDocumentHelper64, windows_core::IUnknown);
impl IDebugDocumentHelper64 {
    pub unsafe fn Init<P0, P1, P2>(&self, pda: P0, pszshortname: P1, pszlongname: P2, docattr: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugApplication64>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pda.param().abi(), pszshortname.param().abi(), pszlongname.param().abi(), docattr).ok() }
    }
    pub unsafe fn Attach<P0>(&self, pddhparent: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentHelper64>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), pddhparent.param().abi()).ok() }
    }
    pub unsafe fn Detach(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddUnicodeText<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUnicodeText)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn AddDBCSText<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDBCSText)(windows_core::Interface::as_raw(self), psztext.param().abi()).ok() }
    }
    pub unsafe fn SetDebugDocumentHost<P0>(&self, pddh: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentHost>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDebugDocumentHost)(windows_core::Interface::as_raw(self), pddh.param().abi()).ok() }
    }
    pub unsafe fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDeferredText)(windows_core::Interface::as_raw(self), cchars, dwtextstartcookie).ok() }
    }
    pub unsafe fn DefineScriptBlock<P2>(&self, ulcharoffset: u32, cchars: u32, pas: P2, fscriptlet: bool) -> windows_core::Result<u64>
    where
        P2: windows_core::Param<IActiveScript>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefineScriptBlock)(windows_core::Interface::as_raw(self), ulcharoffset, cchars, pas.param().abi(), fscriptlet.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultTextAttr(&self, statextattr: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultTextAttr)(windows_core::Interface::as_raw(self), statextattr).ok() }
    }
    pub unsafe fn SetTextAttributes(&self, ulcharoffset: u32, pstatextattr: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTextAttributes)(windows_core::Interface::as_raw(self), ulcharoffset, pstatextattr.len().try_into().unwrap(), core::mem::transmute(pstatextattr.as_ptr())).ok() }
    }
    pub unsafe fn SetLongName<P0>(&self, pszlongname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLongName)(windows_core::Interface::as_raw(self), pszlongname.param().abi()).ok() }
    }
    pub unsafe fn SetShortName<P0>(&self, pszshortname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetShortName)(windows_core::Interface::as_raw(self), pszshortname.param().abi()).ok() }
    }
    pub unsafe fn SetDocumentAttr(&self, pszattributes: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDocumentAttr)(windows_core::Interface::as_raw(self), pszattributes).ok() }
    }
    pub unsafe fn GetDebugApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDebugApplicationNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetScriptBlockInfo(&self, dwsourcecontext: u64, ppasd: *mut Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScriptBlockInfo)(windows_core::Interface::as_raw(self), dwsourcecontext, core::mem::transmute(ppasd), picharpos as _, pcchars as _).ok() }
    }
    pub unsafe fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDebugDocumentContext)(windows_core::Interface::as_raw(self), icharpos, cchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BringDocumentToTop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentToTop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BringDocumentContextToTop<P0>(&self, pddc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).BringDocumentContextToTop)(windows_core::Interface::as_raw(self), pddc.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHelper64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddUnicodeText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddDBCSText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetDebugDocumentHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDeferredText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DefineScriptBlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, windows_core::BOOL, *mut u64) -> windows_core::HRESULT,
    pub SetDefaultTextAttr: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SetTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u16) -> windows_core::HRESULT,
    pub SetLongName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetShortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetDocumentAttr: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDebugApplicationNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScriptBlockInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub CreateDebugDocumentContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BringDocumentToTop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BringDocumentContextToTop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentHelper64_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pda: windows_core::Ref<'_, IDebugApplication64>, pszshortname: &windows_core::PCWSTR, pszlongname: &windows_core::PCWSTR, docattr: u32) -> windows_core::Result<()>;
    fn Attach(&self, pddhparent: windows_core::Ref<'_, IDebugDocumentHelper64>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
    fn AddUnicodeText(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDBCSText(&self, psztext: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetDebugDocumentHost(&self, pddh: windows_core::Ref<'_, IDebugDocumentHost>) -> windows_core::Result<()>;
    fn AddDeferredText(&self, cchars: u32, dwtextstartcookie: u32) -> windows_core::Result<()>;
    fn DefineScriptBlock(&self, ulcharoffset: u32, cchars: u32, pas: windows_core::Ref<'_, IActiveScript>, fscriptlet: windows_core::BOOL) -> windows_core::Result<u64>;
    fn SetDefaultTextAttr(&self, statextattr: u16) -> windows_core::Result<()>;
    fn SetTextAttributes(&self, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> windows_core::Result<()>;
    fn SetLongName(&self, pszlongname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetShortName(&self, pszshortname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDocumentAttr(&self, pszattributes: u32) -> windows_core::Result<()>;
    fn GetDebugApplicationNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(&self, dwsourcecontext: u64, ppasd: windows_core::OutRef<'_, IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::Result<()>;
    fn CreateDebugDocumentContext(&self, icharpos: u32, cchars: u32) -> windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(&self) -> windows_core::Result<()>;
    fn BringDocumentContextToTop(&self, pddc: windows_core::Ref<'_, IDebugDocumentContext>) -> windows_core::Result<()>;
}
impl IDebugDocumentHelper64_Vtbl {
    pub const fn new<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, pszshortname: windows_core::PCWSTR, pszlongname: windows_core::PCWSTR, docattr: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::Init(this, core::mem::transmute_copy(&pda), core::mem::transmute(&pszshortname), core::mem::transmute(&pszlongname), core::mem::transmute_copy(&docattr)).into()
            }
        }
        unsafe extern "system" fn Attach<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddhparent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::Attach(this, core::mem::transmute_copy(&pddhparent)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::Detach(this).into()
            }
        }
        unsafe extern "system" fn AddUnicodeText<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::AddUnicodeText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn AddDBCSText<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::AddDBCSText(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddh: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetDebugDocumentHost(this, core::mem::transmute_copy(&pddh)).into()
            }
        }
        unsafe extern "system" fn AddDeferredText<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::AddDeferredText(this, core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&dwtextstartcookie)).into()
            }
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut core::ffi::c_void, fscriptlet: windows_core::BOOL, pdwsourcecontext: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper64_Impl::DefineScriptBlock(this, core::mem::transmute_copy(&ulcharoffset), core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&pas), core::mem::transmute_copy(&fscriptlet)) {
                    Ok(ok__) => {
                        pdwsourcecontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statextattr: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetDefaultTextAttr(this, core::mem::transmute_copy(&statextattr)).into()
            }
        }
        unsafe extern "system" fn SetTextAttributes<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetTextAttributes(this, core::mem::transmute_copy(&ulcharoffset), core::mem::transmute_copy(&cchars), core::mem::transmute_copy(&pstatextattr)).into()
            }
        }
        unsafe extern "system" fn SetLongName<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlongname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetLongName(this, core::mem::transmute(&pszlongname)).into()
            }
        }
        unsafe extern "system" fn SetShortName<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszshortname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetShortName(this, core::mem::transmute(&pszshortname)).into()
            }
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::SetDocumentAttr(this, core::mem::transmute_copy(&pszattributes)).into()
            }
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdan: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper64_Impl::GetDebugApplicationNode(this) {
                    Ok(ok__) => {
                        ppdan.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsourcecontext: u64, ppasd: *mut *mut core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::GetScriptBlockInfo(this, core::mem::transmute_copy(&dwsourcecontext), core::mem::transmute_copy(&ppasd), core::mem::transmute_copy(&picharpos), core::mem::transmute_copy(&pcchars)).into()
            }
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHelper64_Impl::CreateDebugDocumentContext(this, core::mem::transmute_copy(&icharpos), core::mem::transmute_copy(&cchars)) {
                    Ok(ok__) => {
                        ppddc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::BringDocumentToTop(this).into()
            }
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: IDebugDocumentHelper64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pddc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHelper64_Impl::BringDocumentContextToTop(this, core::mem::transmute_copy(&pddc)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, OFFSET>,
            SetLongName: SetLongName::<Identity, OFFSET>,
            SetShortName: SetShortName::<Identity, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentHelper64 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentHelper64 {}
windows_core::imp::define_interface!(IDebugDocumentHost, IDebugDocumentHost_Vtbl, 0x51973c27_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentHost, windows_core::IUnknown);
impl IDebugDocumentHost {
    pub unsafe fn GetDeferredText(&self, dwtextstartcookie: u32, pchartext: windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeferredText)(windows_core::Interface::as_raw(self), dwtextstartcookie, core::mem::transmute(pchartext), pstatextattr as _, pcnumchars as _, cmaxchars).ok() }
    }
    pub unsafe fn GetScriptTextAttributes<P0, P2>(&self, pstrcode: P0, unumcodechars: u32, pstrdelimiter: P2, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetScriptTextAttributes)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), unumcodechars, pstrdelimiter.param().abi(), dwflags, pattr as _).ok() }
    }
    pub unsafe fn OnCreateDocumentContext(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnCreateDocumentContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPathName(&self, pbstrlongname: *mut windows_core::BSTR, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrlongname), pfisoriginalfile as _).ok() }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NotifyChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeferredText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u16, *mut u32, u32) -> windows_core::HRESULT,
    pub GetScriptTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u16) -> windows_core::HRESULT,
    pub OnCreateDocumentContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentHost_Impl: windows_core::IUnknownImpl {
    fn GetDeferredText(&self, dwtextstartcookie: u32, pchartext: windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::Result<()>;
    fn GetScriptTextAttributes(&self, pstrcode: &windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::Result<()>;
    fn OnCreateDocumentContext(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetPathName(&self, pbstrlongname: *mut windows_core::BSTR, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NotifyChanged(&self) -> windows_core::Result<()>;
}
impl IDebugDocumentHost_Vtbl {
    pub const fn new<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeferredText<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtextstartcookie: u32, pchartext: windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHost_Impl::GetDeferredText(this, core::mem::transmute_copy(&dwtextstartcookie), core::mem::transmute_copy(&pchartext), core::mem::transmute_copy(&pstatextattr), core::mem::transmute_copy(&pcnumchars), core::mem::transmute_copy(&cmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHost_Impl::GetScriptTextAttributes(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&unumcodechars), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)).into()
            }
        }
        unsafe extern "system" fn OnCreateDocumentContext<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkouter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHost_Impl::OnCreateDocumentContext(this) {
                    Ok(ok__) => {
                        ppunkouter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPathName<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlongname: *mut *mut core::ffi::c_void, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHost_Impl::GetPathName(this, core::mem::transmute_copy(&pbstrlongname), core::mem::transmute_copy(&pfisoriginalfile)).into()
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrshortname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentHost_Impl::GetFileName(this) {
                    Ok(ok__) => {
                        pbstrshortname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: IDebugDocumentHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentHost_Impl::NotifyChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeferredText: GetDeferredText::<Identity, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, OFFSET>,
            OnCreateDocumentContext: OnCreateDocumentContext::<Identity, OFFSET>,
            GetPathName: GetPathName::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentHost as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentHost {}
windows_core::imp::define_interface!(IDebugDocumentInfo, IDebugDocumentInfo_Vtbl, 0x51973c1f_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentInfo, windows_core::IUnknown);
impl IDebugDocumentInfo {
    pub unsafe fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), dnt, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDocumentClassId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, DOCUMENTNAMETYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDebugDocumentInfo_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, dnt: DOCUMENTNAMETYPE) -> windows_core::Result<windows_core::BSTR>;
    fn GetDocumentClassId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IDebugDocumentInfo_Vtbl {
    pub const fn new<Identity: IDebugDocumentInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IDebugDocumentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dnt: DOCUMENTNAMETYPE, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentInfo_Impl::GetName(this, core::mem::transmute_copy(&dnt)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentClassId<Identity: IDebugDocumentInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsiddocument: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentInfo_Impl::GetDocumentClassId(this) {
                    Ok(ok__) => {
                        pclsiddocument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetDocumentClassId: GetDocumentClassId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentInfo {}
windows_core::imp::define_interface!(IDebugDocumentProvider, IDebugDocumentProvider_Vtbl, 0x51973c20_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugDocumentProvider {
    type Target = IDebugDocumentInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugDocumentProvider, windows_core::IUnknown, IDebugDocumentInfo);
impl IDebugDocumentProvider {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<IDebugDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentProvider_Vtbl {
    pub base__: IDebugDocumentInfo_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentProvider_Impl: IDebugDocumentInfo_Impl {
    fn GetDocument(&self) -> windows_core::Result<IDebugDocument>;
}
impl IDebugDocumentProvider_Vtbl {
    pub const fn new<Identity: IDebugDocumentProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocument<Identity: IDebugDocumentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppssd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentProvider_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        ppssd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDebugDocumentInfo_Vtbl::new::<Identity, OFFSET>(), GetDocument: GetDocument::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentProvider as windows_core::Interface>::IID || iid == &<IDebugDocumentInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentProvider {}
windows_core::imp::define_interface!(IDebugDocumentText, IDebugDocumentText_Vtbl, 0x51973c22_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugDocumentText {
    type Target = IDebugDocument;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugDocumentText, windows_core::IUnknown, IDebugDocumentInfo, IDebugDocument);
impl IDebugDocumentText {
    pub unsafe fn GetDocumentAttributes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocumentAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pcnumlines as _, pcnumchars as _).ok() }
    }
    pub unsafe fn GetPositionOfLine(&self, clinenumber: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPositionOfLine)(windows_core::Interface::as_raw(self), clinenumber, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLineOfPosition)(windows_core::Interface::as_raw(self), ccharacterposition, pclinenumber as _, pccharacteroffsetinline as _).ok() }
    }
    pub unsafe fn GetText(&self, ccharacterposition: u32, pchartext: windows_core::PWSTR, pstatextattr: Option<*mut u16>, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), ccharacterposition, core::mem::transmute(pchartext), pstatextattr.unwrap_or(core::mem::zeroed()) as _, pcnumchars as _, cmaxchars).ok() }
    }
    pub unsafe fn GetPositionOfContext<P0>(&self, psc: P0, pccharacterposition: *mut u32, cnumchars: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugDocumentContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPositionOfContext)(windows_core::Interface::as_raw(self), psc.param().abi(), pccharacterposition as _, cnumchars as _).ok() }
    }
    pub unsafe fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> windows_core::Result<IDebugDocumentContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextOfPosition)(windows_core::Interface::as_raw(self), ccharacterposition, cnumchars, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentText_Vtbl {
    pub base__: IDebugDocument_Vtbl,
    pub GetDocumentAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPositionOfLine: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetLineOfPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u16, *mut u32, u32) -> windows_core::HRESULT,
    pub GetPositionOfContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetContextOfPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentText_Impl: IDebugDocument_Impl {
    fn GetDocumentAttributes(&self) -> windows_core::Result<u32>;
    fn GetSize(&self, pcnumlines: *mut u32, pcnumchars: *mut u32) -> windows_core::Result<()>;
    fn GetPositionOfLine(&self, clinenumber: u32) -> windows_core::Result<u32>;
    fn GetLineOfPosition(&self, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> windows_core::Result<()>;
    fn GetText(&self, ccharacterposition: u32, pchartext: windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::Result<()>;
    fn GetPositionOfContext(&self, psc: windows_core::Ref<'_, IDebugDocumentContext>, pccharacterposition: *mut u32, cnumchars: *mut u32) -> windows_core::Result<()>;
    fn GetContextOfPosition(&self, ccharacterposition: u32, cnumchars: u32) -> windows_core::Result<IDebugDocumentContext>;
}
impl IDebugDocumentText_Vtbl {
    pub const fn new<Identity: IDebugDocumentText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocumentAttributes<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptextdocattr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentText_Impl::GetDocumentAttributes(this) {
                    Ok(ok__) => {
                        ptextdocattr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnumlines: *mut u32, pcnumchars: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentText_Impl::GetSize(this, core::mem::transmute_copy(&pcnumlines), core::mem::transmute_copy(&pcnumchars)).into()
            }
        }
        unsafe extern "system" fn GetPositionOfLine<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clinenumber: u32, pccharacterposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentText_Impl::GetPositionOfLine(this, core::mem::transmute_copy(&clinenumber)) {
                    Ok(ok__) => {
                        pccharacterposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineOfPosition<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentText_Impl::GetLineOfPosition(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&pclinenumber), core::mem::transmute_copy(&pccharacteroffsetinline)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, pchartext: windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentText_Impl::GetText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&pchartext), core::mem::transmute_copy(&pstatextattr), core::mem::transmute_copy(&pcnumchars), core::mem::transmute_copy(&cmaxchars)).into()
            }
        }
        unsafe extern "system" fn GetPositionOfContext<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psc: *mut core::ffi::c_void, pccharacterposition: *mut u32, cnumchars: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentText_Impl::GetPositionOfContext(this, core::mem::transmute_copy(&psc), core::mem::transmute_copy(&pccharacterposition), core::mem::transmute_copy(&cnumchars)).into()
            }
        }
        unsafe extern "system" fn GetContextOfPosition<Identity: IDebugDocumentText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumchars: u32, ppsc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentText_Impl::GetContextOfPosition(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumchars)) {
                    Ok(ok__) => {
                        ppsc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDebugDocument_Vtbl::new::<Identity, OFFSET>(),
            GetDocumentAttributes: GetDocumentAttributes::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetPositionOfLine: GetPositionOfLine::<Identity, OFFSET>,
            GetLineOfPosition: GetLineOfPosition::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetPositionOfContext: GetPositionOfContext::<Identity, OFFSET>,
            GetContextOfPosition: GetContextOfPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentText as windows_core::Interface>::IID || iid == &<IDebugDocumentInfo as windows_core::Interface>::IID || iid == &<IDebugDocument as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentText {}
windows_core::imp::define_interface!(IDebugDocumentTextAuthor, IDebugDocumentTextAuthor_Vtbl, 0x51973c24_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugDocumentTextAuthor {
    type Target = IDebugDocumentText;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugDocumentTextAuthor, windows_core::IUnknown, IDebugDocumentInfo, IDebugDocument, IDebugDocumentText);
impl IDebugDocumentTextAuthor {
    pub unsafe fn InsertText(&self, ccharacterposition: u32, pchartext: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InsertText)(windows_core::Interface::as_raw(self), ccharacterposition, pchartext.len().try_into().unwrap(), core::mem::transmute(pchartext.as_ptr())).ok() }
    }
    pub unsafe fn RemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveText)(windows_core::Interface::as_raw(self), ccharacterposition, cnumtoremove).ok() }
    }
    pub unsafe fn ReplaceText(&self, ccharacterposition: u32, pchartext: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReplaceText)(windows_core::Interface::as_raw(self), ccharacterposition, pchartext.len().try_into().unwrap(), core::mem::transmute(pchartext.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextAuthor_Vtbl {
    pub base__: IDebugDocumentText_Vtbl,
    pub InsertText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReplaceText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IDebugDocumentTextAuthor_Impl: IDebugDocumentText_Impl {
    fn InsertText(&self, ccharacterposition: u32, cnumtoinsert: u32, pchartext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::Result<()>;
    fn ReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32, pchartext: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IDebugDocumentTextAuthor_Vtbl {
    pub const fn new<Identity: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InsertText<Identity: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32, pchartext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextAuthor_Impl::InsertText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoinsert), core::mem::transmute(&pchartext)).into()
            }
        }
        unsafe extern "system" fn RemoveText<Identity: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextAuthor_Impl::RemoveText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoremove)).into()
            }
        }
        unsafe extern "system" fn ReplaceText<Identity: IDebugDocumentTextAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32, pchartext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextAuthor_Impl::ReplaceText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoreplace), core::mem::transmute(&pchartext)).into()
            }
        }
        Self {
            base__: IDebugDocumentText_Vtbl::new::<Identity, OFFSET>(),
            InsertText: InsertText::<Identity, OFFSET>,
            RemoveText: RemoveText::<Identity, OFFSET>,
            ReplaceText: ReplaceText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentTextAuthor as windows_core::Interface>::IID || iid == &<IDebugDocumentInfo as windows_core::Interface>::IID || iid == &<IDebugDocument as windows_core::Interface>::IID || iid == &<IDebugDocumentText as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentTextAuthor {}
windows_core::imp::define_interface!(IDebugDocumentTextEvents, IDebugDocumentTextEvents_Vtbl, 0x51973c23_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentTextEvents, windows_core::IUnknown);
impl IDebugDocumentTextEvents {
    pub unsafe fn onDestroy(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onDestroy)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn onInsertText(&self, ccharacterposition: u32, cnumtoinsert: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onInsertText)(windows_core::Interface::as_raw(self), ccharacterposition, cnumtoinsert).ok() }
    }
    pub unsafe fn onRemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onRemoveText)(windows_core::Interface::as_raw(self), ccharacterposition, cnumtoremove).ok() }
    }
    pub unsafe fn onReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onReplaceText)(windows_core::Interface::as_raw(self), ccharacterposition, cnumtoreplace).ok() }
    }
    pub unsafe fn onUpdateTextAttributes(&self, ccharacterposition: u32, cnumtoupdate: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onUpdateTextAttributes)(windows_core::Interface::as_raw(self), ccharacterposition, cnumtoupdate).ok() }
    }
    pub unsafe fn onUpdateDocumentAttributes(&self, textdocattr: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onUpdateDocumentAttributes)(windows_core::Interface::as_raw(self), textdocattr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub onDestroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub onInsertText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub onRemoveText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub onReplaceText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub onUpdateTextAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub onUpdateDocumentAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDebugDocumentTextEvents_Impl: windows_core::IUnknownImpl {
    fn onDestroy(&self) -> windows_core::Result<()>;
    fn onInsertText(&self, ccharacterposition: u32, cnumtoinsert: u32) -> windows_core::Result<()>;
    fn onRemoveText(&self, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::Result<()>;
    fn onReplaceText(&self, ccharacterposition: u32, cnumtoreplace: u32) -> windows_core::Result<()>;
    fn onUpdateTextAttributes(&self, ccharacterposition: u32, cnumtoupdate: u32) -> windows_core::Result<()>;
    fn onUpdateDocumentAttributes(&self, textdocattr: u32) -> windows_core::Result<()>;
}
impl IDebugDocumentTextEvents_Vtbl {
    pub const fn new<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn onDestroy<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onDestroy(this).into()
            }
        }
        unsafe extern "system" fn onInsertText<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onInsertText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoinsert)).into()
            }
        }
        unsafe extern "system" fn onRemoveText<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onRemoveText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoremove)).into()
            }
        }
        unsafe extern "system" fn onReplaceText<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onReplaceText(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoreplace)).into()
            }
        }
        unsafe extern "system" fn onUpdateTextAttributes<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccharacterposition: u32, cnumtoupdate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onUpdateTextAttributes(this, core::mem::transmute_copy(&ccharacterposition), core::mem::transmute_copy(&cnumtoupdate)).into()
            }
        }
        unsafe extern "system" fn onUpdateDocumentAttributes<Identity: IDebugDocumentTextEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textdocattr: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextEvents_Impl::onUpdateDocumentAttributes(this, core::mem::transmute_copy(&textdocattr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onDestroy: onDestroy::<Identity, OFFSET>,
            onInsertText: onInsertText::<Identity, OFFSET>,
            onRemoveText: onRemoveText::<Identity, OFFSET>,
            onReplaceText: onReplaceText::<Identity, OFFSET>,
            onUpdateTextAttributes: onUpdateTextAttributes::<Identity, OFFSET>,
            onUpdateDocumentAttributes: onUpdateDocumentAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentTextEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentTextEvents {}
windows_core::imp::define_interface!(IDebugDocumentTextExternalAuthor, IDebugDocumentTextExternalAuthor_Vtbl, 0x51973c25_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugDocumentTextExternalAuthor, windows_core::IUnknown);
impl IDebugDocumentTextExternalAuthor {
    pub unsafe fn GetPathName(&self, pbstrlongname: *mut windows_core::BSTR, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPathName)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrlongname), pfisoriginalfile as _).ok() }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NotifyChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugDocumentTextExternalAuthor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPathName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugDocumentTextExternalAuthor_Impl: windows_core::IUnknownImpl {
    fn GetPathName(&self, pbstrlongname: *mut windows_core::BSTR, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NotifyChanged(&self) -> windows_core::Result<()>;
}
impl IDebugDocumentTextExternalAuthor_Vtbl {
    pub const fn new<Identity: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPathName<Identity: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlongname: *mut *mut core::ffi::c_void, pfisoriginalfile: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextExternalAuthor_Impl::GetPathName(this, core::mem::transmute_copy(&pbstrlongname), core::mem::transmute_copy(&pfisoriginalfile)).into()
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrshortname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugDocumentTextExternalAuthor_Impl::GetFileName(this) {
                    Ok(ok__) => {
                        pbstrshortname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugDocumentTextExternalAuthor_Impl::NotifyChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPathName: GetPathName::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugDocumentTextExternalAuthor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugDocumentTextExternalAuthor {}
windows_core::imp::define_interface!(IDebugExpression, IDebugExpression_Vtbl, 0x51973c14_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugExpression, windows_core::IUnknown);
impl IDebugExpression {
    pub unsafe fn Start<P0>(&self, pdecb: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugExpressionCallBack>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pdecb.param().abi()).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn QueryIsComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryIsComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetResultAsString(&self, phrresult: *mut windows_core::HRESULT, pbstrresult: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResultAsString)(windows_core::Interface::as_raw(self), phrresult as _, core::mem::transmute(pbstrresult)).ok() }
    }
    pub unsafe fn GetResultAsDebugProperty(&self, phrresult: *mut windows_core::HRESULT, ppdp: *mut Option<super::IDebugProperty>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResultAsDebugProperty)(windows_core::Interface::as_raw(self), phrresult as _, core::mem::transmute(ppdp)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpression_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryIsComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResultAsDebugProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugExpression_Impl: windows_core::IUnknownImpl {
    fn Start(&self, pdecb: windows_core::Ref<'_, IDebugExpressionCallBack>) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn QueryIsComplete(&self) -> windows_core::Result<()>;
    fn GetResultAsString(&self, phrresult: *mut windows_core::HRESULT, pbstrresult: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetResultAsDebugProperty(&self, phrresult: *mut windows_core::HRESULT, ppdp: windows_core::OutRef<'_, super::IDebugProperty>) -> windows_core::Result<()>;
}
impl IDebugExpression_Vtbl {
    pub const fn new<Identity: IDebugExpression_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IDebugExpression_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpression_Impl::Start(this, core::mem::transmute_copy(&pdecb)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IDebugExpression_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpression_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn QueryIsComplete<Identity: IDebugExpression_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpression_Impl::QueryIsComplete(this).into()
            }
        }
        unsafe extern "system" fn GetResultAsString<Identity: IDebugExpression_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, pbstrresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpression_Impl::GetResultAsString(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&pbstrresult)).into()
            }
        }
        unsafe extern "system" fn GetResultAsDebugProperty<Identity: IDebugExpression_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, ppdp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpression_Impl::GetResultAsDebugProperty(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&ppdp)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, OFFSET>,
            GetResultAsString: GetResultAsString::<Identity, OFFSET>,
            GetResultAsDebugProperty: GetResultAsDebugProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugExpression as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugExpression {}
windows_core::imp::define_interface!(IDebugExpressionCallBack, IDebugExpressionCallBack_Vtbl, 0x51973c16_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugExpressionCallBack, windows_core::IUnknown);
impl IDebugExpressionCallBack {
    pub unsafe fn onComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).onComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpressionCallBack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub onComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugExpressionCallBack_Impl: windows_core::IUnknownImpl {
    fn onComplete(&self) -> windows_core::Result<()>;
}
impl IDebugExpressionCallBack_Vtbl {
    pub const fn new<Identity: IDebugExpressionCallBack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn onComplete<Identity: IDebugExpressionCallBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpressionCallBack_Impl::onComplete(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugExpressionCallBack as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugExpressionCallBack {}
windows_core::imp::define_interface!(IDebugExpressionContext, IDebugExpressionContext_Vtbl, 0x51973c15_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugExpressionContext, windows_core::IUnknown);
impl IDebugExpressionContext {
    pub unsafe fn ParseLanguageText<P0, P2>(&self, pstrcode: P0, nradix: u32, pstrdelimiter: P2, dwflags: u32) -> windows_core::Result<IDebugExpression>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseLanguageText)(windows_core::Interface::as_raw(self), pstrcode.param().abi(), nradix, pstrdelimiter.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLanguageInfo(&self, pbstrlanguagename: *mut windows_core::BSTR, planguageid: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLanguageInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrlanguagename), planguageid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExpressionContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseLanguageText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLanguageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDebugExpressionContext_Impl: windows_core::IUnknownImpl {
    fn ParseLanguageText(&self, pstrcode: &windows_core::PCWSTR, nradix: u32, pstrdelimiter: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<IDebugExpression>;
    fn GetLanguageInfo(&self, pbstrlanguagename: *mut windows_core::BSTR, planguageid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IDebugExpressionContext_Vtbl {
    pub const fn new<Identity: IDebugExpressionContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ParseLanguageText<Identity: IDebugExpressionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcode: windows_core::PCWSTR, nradix: u32, pstrdelimiter: windows_core::PCWSTR, dwflags: u32, ppe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugExpressionContext_Impl::ParseLanguageText(this, core::mem::transmute(&pstrcode), core::mem::transmute_copy(&nradix), core::mem::transmute(&pstrdelimiter), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguageInfo<Identity: IDebugExpressionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlanguagename: *mut *mut core::ffi::c_void, planguageid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExpressionContext_Impl::GetLanguageInfo(this, core::mem::transmute_copy(&pbstrlanguagename), core::mem::transmute_copy(&planguageid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseLanguageText: ParseLanguageText::<Identity, OFFSET>,
            GetLanguageInfo: GetLanguageInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugExpressionContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugExpressionContext {}
windows_core::imp::define_interface!(IDebugFormatter, IDebugFormatter_Vtbl, 0x51973c05_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugFormatter, windows_core::IUnknown);
impl IDebugFormatter {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetStringForVariant(&self, pvar: *const super::super::super::Variant::VARIANT, nradix: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringForVariant)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), nradix, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetVariantForString<P0>(&self, pwstrvalue: P0) -> windows_core::Result<super::super::super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVariantForString)(windows_core::Interface::as_raw(self), pwstrvalue.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetStringForVarType(&self, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringForVarType)(windows_core::Interface::as_raw(self), vt, ptdescarraytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugFormatter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetStringForVariant: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Variant::VARIANT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetStringForVariant: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetVariantForString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetVariantForString: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetStringForVarType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Variant::VARENUM, *const super::super::super::Com::TYPEDESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetStringForVarType: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugFormatter_Impl: windows_core::IUnknownImpl {
    fn GetStringForVariant(&self, pvar: *const super::super::super::Variant::VARIANT, nradix: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetVariantForString(&self, pwstrvalue: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Variant::VARIANT>;
    fn GetStringForVarType(&self, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDebugFormatter_Vtbl {
    pub const fn new<Identity: IDebugFormatter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringForVariant<Identity: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, nradix: u32, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugFormatter_Impl::GetStringForVariant(this, core::mem::transmute_copy(&pvar), core::mem::transmute_copy(&nradix)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVariantForString<Identity: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrvalue: windows_core::PCWSTR, pvar: *mut super::super::super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugFormatter_Impl::GetVariantForString(this, core::mem::transmute(&pwstrvalue)) {
                    Ok(ok__) => {
                        pvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringForVarType<Identity: IDebugFormatter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugFormatter_Impl::GetStringForVarType(this, core::mem::transmute_copy(&vt), core::mem::transmute_copy(&ptdescarraytype)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringForVariant: GetStringForVariant::<Identity, OFFSET>,
            GetVariantForString: GetVariantForString::<Identity, OFFSET>,
            GetStringForVarType: GetStringForVarType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugFormatter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDebugFormatter {}
windows_core::imp::define_interface!(IDebugHelper, IDebugHelper_Vtbl, 0x51973c3f_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugHelper, windows_core::IUnknown);
impl IDebugHelper {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreatePropertyBrowser<P1, P2>(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: P1, pdat: P2) -> windows_core::Result<super::IDebugProperty>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IDebugApplicationThread>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyBrowser)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), bstrname.param().abi(), pdat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CreatePropertyBrowserEx<P1, P2, P3>(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: P1, pdat: P2, pdf: P3) -> windows_core::Result<super::IDebugProperty>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<IDebugApplicationThread>,
        P3: windows_core::Param<IDebugFormatter>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyBrowserEx)(windows_core::Interface::as_raw(self), core::mem::transmute(pvar), bstrname.param().abi(), pdat.param().abi(), pdf.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSimpleConnectionPoint<P0>(&self, pdisp: P0) -> windows_core::Result<ISimpleConnectionPoint>
    where
        P0: windows_core::Param<super::super::super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSimpleConnectionPoint)(windows_core::Interface::as_raw(self), pdisp.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreatePropertyBrowser: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Variant::VARIANT, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreatePropertyBrowser: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CreatePropertyBrowserEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::super::Variant::VARIANT, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CreatePropertyBrowserEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSimpleConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSimpleConnectionPoint: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugHelper_Impl: windows_core::IUnknownImpl {
    fn CreatePropertyBrowser(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: &windows_core::PCWSTR, pdat: windows_core::Ref<'_, IDebugApplicationThread>) -> windows_core::Result<super::IDebugProperty>;
    fn CreatePropertyBrowserEx(&self, pvar: *const super::super::super::Variant::VARIANT, bstrname: &windows_core::PCWSTR, pdat: windows_core::Ref<'_, IDebugApplicationThread>, pdf: windows_core::Ref<'_, IDebugFormatter>) -> windows_core::Result<super::IDebugProperty>;
    fn CreateSimpleConnectionPoint(&self, pdisp: windows_core::Ref<'_, super::super::super::Com::IDispatch>) -> windows_core::Result<ISimpleConnectionPoint>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDebugHelper_Vtbl {
    pub const fn new<Identity: IDebugHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePropertyBrowser<Identity: IDebugHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: windows_core::PCWSTR, pdat: *mut core::ffi::c_void, ppdob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugHelper_Impl::CreatePropertyBrowser(this, core::mem::transmute_copy(&pvar), core::mem::transmute(&bstrname), core::mem::transmute_copy(&pdat)) {
                    Ok(ok__) => {
                        ppdob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyBrowserEx<Identity: IDebugHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: windows_core::PCWSTR, pdat: *mut core::ffi::c_void, pdf: *mut core::ffi::c_void, ppdob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugHelper_Impl::CreatePropertyBrowserEx(this, core::mem::transmute_copy(&pvar), core::mem::transmute(&bstrname), core::mem::transmute_copy(&pdat), core::mem::transmute_copy(&pdf)) {
                    Ok(ok__) => {
                        ppdob.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSimpleConnectionPoint<Identity: IDebugHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisp: *mut core::ffi::c_void, ppscp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugHelper_Impl::CreateSimpleConnectionPoint(this, core::mem::transmute_copy(&pdisp)) {
                    Ok(ok__) => {
                        ppscp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyBrowser: CreatePropertyBrowser::<Identity, OFFSET>,
            CreatePropertyBrowserEx: CreatePropertyBrowserEx::<Identity, OFFSET>,
            CreateSimpleConnectionPoint: CreateSimpleConnectionPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDebugHelper {}
windows_core::imp::define_interface!(IDebugSessionProvider, IDebugSessionProvider_Vtbl, 0x51973c29_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugSessionProvider, windows_core::IUnknown);
impl IDebugSessionProvider {
    pub unsafe fn StartDebugSession<P0>(&self, pda: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplication>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartDebugSession)(windows_core::Interface::as_raw(self), pda.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSessionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartDebugSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugSessionProvider_Impl: windows_core::IUnknownImpl {
    fn StartDebugSession(&self, pda: windows_core::Ref<'_, IRemoteDebugApplication>) -> windows_core::Result<()>;
}
impl IDebugSessionProvider_Vtbl {
    pub const fn new<Identity: IDebugSessionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartDebugSession<Identity: IDebugSessionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugSessionProvider_Impl::StartDebugSession(this, core::mem::transmute_copy(&pda)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartDebugSession: StartDebugSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugSessionProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugSessionProvider {}
windows_core::imp::define_interface!(IDebugStackFrame, IDebugStackFrame_Vtbl, 0x51973c17_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugStackFrame, windows_core::IUnknown);
impl IDebugStackFrame {
    pub unsafe fn GetCodeContext(&self) -> windows_core::Result<IDebugCodeContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodeContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDescriptionString(&self, flong: bool) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescriptionString)(windows_core::Interface::as_raw(self), flong.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLanguageString(&self, flong: bool) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageString)(windows_core::Interface::as_raw(self), flong.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetThread(&self) -> windows_core::Result<IDebugApplicationThread> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDebugProperty(&self) -> windows_core::Result<super::IDebugProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDebugProperty)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCodeContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescriptionString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLanguageString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDebugProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugStackFrame_Impl: windows_core::IUnknownImpl {
    fn GetCodeContext(&self) -> windows_core::Result<IDebugCodeContext>;
    fn GetDescriptionString(&self, flong: windows_core::BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn GetLanguageString(&self, flong: windows_core::BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn GetThread(&self) -> windows_core::Result<IDebugApplicationThread>;
    fn GetDebugProperty(&self) -> windows_core::Result<super::IDebugProperty>;
}
impl IDebugStackFrame_Vtbl {
    pub const fn new<Identity: IDebugStackFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCodeContext<Identity: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame_Impl::GetCodeContext(this) {
                    Ok(ok__) => {
                        ppcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescriptionString<Identity: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flong: windows_core::BOOL, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame_Impl::GetDescriptionString(this, core::mem::transmute_copy(&flong)) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguageString<Identity: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flong: windows_core::BOOL, pbstrlanguage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame_Impl::GetLanguageString(this, core::mem::transmute_copy(&flong)) {
                    Ok(ok__) => {
                        pbstrlanguage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThread<Identity: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame_Impl::GetThread(this) {
                    Ok(ok__) => {
                        ppat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDebugProperty<Identity: IDebugStackFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdebugprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame_Impl::GetDebugProperty(this) {
                    Ok(ok__) => {
                        ppdebugprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodeContext: GetCodeContext::<Identity, OFFSET>,
            GetDescriptionString: GetDescriptionString::<Identity, OFFSET>,
            GetLanguageString: GetLanguageString::<Identity, OFFSET>,
            GetThread: GetThread::<Identity, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugStackFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugStackFrame {}
windows_core::imp::define_interface!(IDebugStackFrame110, IDebugStackFrame110_Vtbl, 0x4b509611_b6ea_4b24_adcb_d0ccfd1a7e33);
impl core::ops::Deref for IDebugStackFrame110 {
    type Target = IDebugStackFrame;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugStackFrame110, windows_core::IUnknown, IDebugStackFrame);
impl IDebugStackFrame110 {
    pub unsafe fn GetStackFrameType(&self) -> windows_core::Result<DEBUG_STACKFRAME_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStackFrameType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScriptInvocationContext(&self) -> windows_core::Result<IScriptInvocationContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScriptInvocationContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrame110_Vtbl {
    pub base__: IDebugStackFrame_Vtbl,
    pub GetStackFrameType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEBUG_STACKFRAME_TYPE) -> windows_core::HRESULT,
    pub GetScriptInvocationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugStackFrame110_Impl: IDebugStackFrame_Impl {
    fn GetStackFrameType(&self) -> windows_core::Result<DEBUG_STACKFRAME_TYPE>;
    fn GetScriptInvocationContext(&self) -> windows_core::Result<IScriptInvocationContext>;
}
impl IDebugStackFrame110_Vtbl {
    pub const fn new<Identity: IDebugStackFrame110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStackFrameType<Identity: IDebugStackFrame110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstackframekind: *mut DEBUG_STACKFRAME_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame110_Impl::GetStackFrameType(this) {
                    Ok(ok__) => {
                        pstackframekind.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptInvocationContext<Identity: IDebugStackFrame110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinvocationcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrame110_Impl::GetScriptInvocationContext(this) {
                    Ok(ok__) => {
                        ppinvocationcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDebugStackFrame_Vtbl::new::<Identity, OFFSET>(),
            GetStackFrameType: GetStackFrameType::<Identity, OFFSET>,
            GetScriptInvocationContext: GetScriptInvocationContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugStackFrame110 as windows_core::Interface>::IID || iid == &<IDebugStackFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugStackFrame110 {}
windows_core::imp::define_interface!(IDebugStackFrameSniffer, IDebugStackFrameSniffer_Vtbl, 0x51973c18_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugStackFrameSniffer, windows_core::IUnknown);
impl IDebugStackFrameSniffer {
    pub unsafe fn EnumStackFrames(&self) -> windows_core::Result<IEnumDebugStackFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStackFrames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSniffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumStackFrames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugStackFrameSniffer_Impl: windows_core::IUnknownImpl {
    fn EnumStackFrames(&self) -> windows_core::Result<IEnumDebugStackFrames>;
}
impl IDebugStackFrameSniffer_Vtbl {
    pub const fn new<Identity: IDebugStackFrameSniffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumStackFrames<Identity: IDebugStackFrameSniffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrameSniffer_Impl::EnumStackFrames(this) {
                    Ok(ok__) => {
                        ppedsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumStackFrames: EnumStackFrames::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugStackFrameSniffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugStackFrameSniffer {}
windows_core::imp::define_interface!(IDebugStackFrameSnifferEx32, IDebugStackFrameSnifferEx32_Vtbl, 0x51973c19_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugStackFrameSnifferEx32 {
    type Target = IDebugStackFrameSniffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugStackFrameSnifferEx32, windows_core::IUnknown, IDebugStackFrameSniffer);
impl IDebugStackFrameSnifferEx32 {
    pub unsafe fn EnumStackFramesEx32(&self, dwspmin: u32) -> windows_core::Result<IEnumDebugStackFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStackFramesEx32)(windows_core::Interface::as_raw(self), dwspmin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSnifferEx32_Vtbl {
    pub base__: IDebugStackFrameSniffer_Vtbl,
    pub EnumStackFramesEx32: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugStackFrameSnifferEx32_Impl: IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx32(&self, dwspmin: u32) -> windows_core::Result<IEnumDebugStackFrames>;
}
impl IDebugStackFrameSnifferEx32_Vtbl {
    pub const fn new<Identity: IDebugStackFrameSnifferEx32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumStackFramesEx32<Identity: IDebugStackFrameSnifferEx32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspmin: u32, ppedsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrameSnifferEx32_Impl::EnumStackFramesEx32(this, core::mem::transmute_copy(&dwspmin)) {
                    Ok(ok__) => {
                        ppedsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDebugStackFrameSniffer_Vtbl::new::<Identity, OFFSET>(), EnumStackFramesEx32: EnumStackFramesEx32::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugStackFrameSnifferEx32 as windows_core::Interface>::IID || iid == &<IDebugStackFrameSniffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugStackFrameSnifferEx32 {}
windows_core::imp::define_interface!(IDebugStackFrameSnifferEx64, IDebugStackFrameSnifferEx64_Vtbl, 0x8cd12af4_49c1_4d52_8d8a_c146f47581aa);
impl core::ops::Deref for IDebugStackFrameSnifferEx64 {
    type Target = IDebugStackFrameSniffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugStackFrameSnifferEx64, windows_core::IUnknown, IDebugStackFrameSniffer);
impl IDebugStackFrameSnifferEx64 {
    pub unsafe fn EnumStackFramesEx64(&self, dwspmin: u64) -> windows_core::Result<IEnumDebugStackFrames64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStackFramesEx64)(windows_core::Interface::as_raw(self), dwspmin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugStackFrameSnifferEx64_Vtbl {
    pub base__: IDebugStackFrameSniffer_Vtbl,
    pub EnumStackFramesEx64: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugStackFrameSnifferEx64_Impl: IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx64(&self, dwspmin: u64) -> windows_core::Result<IEnumDebugStackFrames64>;
}
impl IDebugStackFrameSnifferEx64_Vtbl {
    pub const fn new<Identity: IDebugStackFrameSnifferEx64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumStackFramesEx64<Identity: IDebugStackFrameSnifferEx64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspmin: u64, ppedsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugStackFrameSnifferEx64_Impl::EnumStackFramesEx64(this, core::mem::transmute_copy(&dwspmin)) {
                    Ok(ok__) => {
                        ppedsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDebugStackFrameSniffer_Vtbl::new::<Identity, OFFSET>(), EnumStackFramesEx64: EnumStackFramesEx64::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugStackFrameSnifferEx64 as windows_core::Interface>::IID || iid == &<IDebugStackFrameSniffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugStackFrameSnifferEx64 {}
windows_core::imp::define_interface!(IDebugSyncOperation, IDebugSyncOperation_Vtbl, 0x51973c1a_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugSyncOperation, windows_core::IUnknown);
impl IDebugSyncOperation {
    pub unsafe fn GetTargetThread(&self) -> windows_core::Result<IDebugApplicationThread> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Execute(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InProgressAbort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InProgressAbort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugSyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InProgressAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugSyncOperation_Impl: windows_core::IUnknownImpl {
    fn GetTargetThread(&self) -> windows_core::Result<IDebugApplicationThread>;
    fn Execute(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn InProgressAbort(&self) -> windows_core::Result<()>;
}
impl IDebugSyncOperation_Vtbl {
    pub const fn new<Identity: IDebugSyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTargetThread<Identity: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugSyncOperation_Impl::GetTargetThread(this) {
                    Ok(ok__) => {
                        ppattarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Execute<Identity: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugSyncOperation_Impl::Execute(this) {
                    Ok(ok__) => {
                        ppunkresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InProgressAbort<Identity: IDebugSyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugSyncOperation_Impl::InProgressAbort(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTargetThread: GetTargetThread::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
            InProgressAbort: InProgressAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugSyncOperation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugSyncOperation {}
windows_core::imp::define_interface!(IDebugThreadCall32, IDebugThreadCall32_Vtbl, 0x51973c36_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugThreadCall32, windows_core::IUnknown);
impl IDebugThreadCall32 {
    pub unsafe fn ThreadCallHandler(&self, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadCallHandler)(windows_core::Interface::as_raw(self), dwparam1, dwparam2, dwparam3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugThreadCall32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadCallHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
}
pub trait IDebugThreadCall32_Impl: windows_core::IUnknownImpl {
    fn ThreadCallHandler(&self, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::Result<()>;
}
impl IDebugThreadCall32_Vtbl {
    pub const fn new<Identity: IDebugThreadCall32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadCallHandler<Identity: IDebugThreadCall32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugThreadCall32_Impl::ThreadCallHandler(this, core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThreadCallHandler: ThreadCallHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugThreadCall32 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugThreadCall32 {}
windows_core::imp::define_interface!(IDebugThreadCall64, IDebugThreadCall64_Vtbl, 0xcb3fa335_e979_42fd_9fcf_a7546a0f3905);
windows_core::imp::interface_hierarchy!(IDebugThreadCall64, windows_core::IUnknown);
impl IDebugThreadCall64 {
    pub unsafe fn ThreadCallHandler(&self, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ThreadCallHandler)(windows_core::Interface::as_raw(self), dwparam1, dwparam2, dwparam3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugThreadCall64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ThreadCallHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u64) -> windows_core::HRESULT,
}
pub trait IDebugThreadCall64_Impl: windows_core::IUnknownImpl {
    fn ThreadCallHandler(&self, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::Result<()>;
}
impl IDebugThreadCall64_Vtbl {
    pub const fn new<Identity: IDebugThreadCall64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ThreadCallHandler<Identity: IDebugThreadCall64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugThreadCall64_Impl::ThreadCallHandler(this, core::mem::transmute_copy(&dwparam1), core::mem::transmute_copy(&dwparam2), core::mem::transmute_copy(&dwparam3)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ThreadCallHandler: ThreadCallHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugThreadCall64 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugThreadCall64 {}
windows_core::imp::define_interface!(IEnumDebugApplicationNodes, IEnumDebugApplicationNodes_Vtbl, 0x51973c3a_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugApplicationNodes, windows_core::IUnknown);
impl IEnumDebugApplicationNodes {
    pub unsafe fn Next(&self, celt: u32, pprddp: *mut Option<IDebugApplicationNode>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(pprddp), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDebugApplicationNodes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugApplicationNodes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDebugApplicationNodes_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pprddp: windows_core::OutRef<'_, IDebugApplicationNode>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugApplicationNodes>;
}
impl IEnumDebugApplicationNodes_Vtbl {
    pub const fn new<Identity: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pprddp: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugApplicationNodes_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pprddp), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugApplicationNodes_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugApplicationNodes_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugApplicationNodes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperddp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugApplicationNodes_Impl::Clone(this) {
                    Ok(ok__) => {
                        pperddp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugApplicationNodes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugApplicationNodes {}
windows_core::imp::define_interface!(IEnumDebugCodeContexts, IEnumDebugCodeContexts_Vtbl, 0x51973c1d_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugCodeContexts, windows_core::IUnknown);
impl IEnumDebugCodeContexts {
    pub unsafe fn Next(&self, celt: u32, pscc: *mut Option<IDebugCodeContext>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(pscc), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDebugCodeContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugCodeContexts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDebugCodeContexts_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pscc: windows_core::OutRef<'_, IDebugCodeContext>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugCodeContexts>;
}
impl IEnumDebugCodeContexts_Vtbl {
    pub const fn new<Identity: IEnumDebugCodeContexts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pscc: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugCodeContexts_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pscc), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugCodeContexts_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugCodeContexts_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugCodeContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppescc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugCodeContexts_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppescc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugCodeContexts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugCodeContexts {}
windows_core::imp::define_interface!(IEnumDebugExpressionContexts, IEnumDebugExpressionContexts_Vtbl, 0x51973c40_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugExpressionContexts, windows_core::IUnknown);
impl IEnumDebugExpressionContexts {
    pub unsafe fn Next(&self, celt: u32, ppdec: *mut Option<IDebugExpressionContext>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppdec), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDebugExpressionContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugExpressionContexts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDebugExpressionContexts_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppdec: windows_core::OutRef<'_, IDebugExpressionContext>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugExpressionContexts>;
}
impl IEnumDebugExpressionContexts_Vtbl {
    pub const fn new<Identity: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppdec: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExpressionContexts_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppdec), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExpressionContexts_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExpressionContexts_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugExpressionContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedec: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugExpressionContexts_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppedec.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugExpressionContexts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugExpressionContexts {}
windows_core::imp::define_interface!(IEnumDebugStackFrames, IEnumDebugStackFrames_Vtbl, 0x51973c1e_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugStackFrames, windows_core::IUnknown);
impl IEnumDebugStackFrames {
    pub unsafe fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(prgdsfd), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumDebugStackFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugStackFrames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DebugStackFrameDescriptor, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumDebugStackFrames_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugStackFrames>;
}
impl IEnumDebugStackFrames_Vtbl {
    pub const fn new<Identity: IEnumDebugStackFrames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugStackFrames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&prgdsfd), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugStackFrames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugStackFrames_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugStackFrames_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppedsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugStackFrames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugStackFrames {}
windows_core::imp::define_interface!(IEnumDebugStackFrames64, IEnumDebugStackFrames64_Vtbl, 0x0dc38853_c1b0_4176_a984_b298361027af);
impl core::ops::Deref for IEnumDebugStackFrames64 {
    type Target = IEnumDebugStackFrames;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumDebugStackFrames64, windows_core::IUnknown, IEnumDebugStackFrames);
impl IEnumDebugStackFrames64 {
    pub unsafe fn Next64(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next64)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(prgdsfd), pceltfetched as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugStackFrames64_Vtbl {
    pub base__: IEnumDebugStackFrames_Vtbl,
    pub Next64: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DebugStackFrameDescriptor64, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumDebugStackFrames64_Impl: IEnumDebugStackFrames_Impl {
    fn Next64(&self, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl IEnumDebugStackFrames64_Vtbl {
    pub const fn new<Identity: IEnumDebugStackFrames64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next64<Identity: IEnumDebugStackFrames64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugStackFrames64_Impl::Next64(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&prgdsfd), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        Self { base__: IEnumDebugStackFrames_Vtbl::new::<Identity, OFFSET>(), Next64: Next64::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugStackFrames64 as windows_core::Interface>::IID || iid == &<IEnumDebugStackFrames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugStackFrames64 {}
windows_core::imp::define_interface!(IEnumJsStackFrames, IEnumJsStackFrames_Vtbl, 0x5e7da34b_fb51_4791_abe7_cb5bdf419755);
windows_core::imp::interface_hierarchy!(IEnumJsStackFrames, windows_core::IUnknown);
impl IEnumJsStackFrames {
    pub unsafe fn Next(&self, pframes: &mut [JS_NATIVE_FRAME], pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pframes.len().try_into().unwrap(), core::mem::transmute(pframes.as_ptr()), pcfetched as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumJsStackFrames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut JS_NATIVE_FRAME, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumJsStackFrames_Impl: windows_core::IUnknownImpl {
    fn Next(&self, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IEnumJsStackFrames_Vtbl {
    pub const fn new<Identity: IEnumJsStackFrames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumJsStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumJsStackFrames_Impl::Next(this, core::mem::transmute_copy(&cframecount), core::mem::transmute_copy(&pframes), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumJsStackFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumJsStackFrames_Impl::Reset(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET>, Reset: Reset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumJsStackFrames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumJsStackFrames {}
windows_core::imp::define_interface!(IEnumRemoteDebugApplicationThreads, IEnumRemoteDebugApplicationThreads_Vtbl, 0x51973c3c_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumRemoteDebugApplicationThreads, windows_core::IUnknown);
impl IEnumRemoteDebugApplicationThreads {
    pub unsafe fn Next(&self, celt: u32, pprdat: *mut Option<IRemoteDebugApplicationThread>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(pprdat), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRemoteDebugApplicationThreads> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRemoteDebugApplicationThreads_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumRemoteDebugApplicationThreads_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pprdat: windows_core::OutRef<'_, IRemoteDebugApplicationThread>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumRemoteDebugApplicationThreads>;
}
impl IEnumRemoteDebugApplicationThreads_Vtbl {
    pub const fn new<Identity: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pprdat: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplicationThreads_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pprdat), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplicationThreads_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplicationThreads_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperdat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumRemoteDebugApplicationThreads_Impl::Clone(this) {
                    Ok(ok__) => {
                        pperdat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRemoteDebugApplicationThreads as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumRemoteDebugApplicationThreads {}
windows_core::imp::define_interface!(IEnumRemoteDebugApplications, IEnumRemoteDebugApplications_Vtbl, 0x51973c3b_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumRemoteDebugApplications, windows_core::IUnknown);
impl IEnumRemoteDebugApplications {
    pub unsafe fn Next(&self, celt: u32, ppda: *mut Option<IRemoteDebugApplication>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppda), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRemoteDebugApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRemoteDebugApplications_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumRemoteDebugApplications_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppda: windows_core::OutRef<'_, IRemoteDebugApplication>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumRemoteDebugApplications>;
}
impl IEnumRemoteDebugApplications_Vtbl {
    pub const fn new<Identity: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppda: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplications_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppda), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplications_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRemoteDebugApplications_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumRemoteDebugApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppessd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumRemoteDebugApplications_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppessd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRemoteDebugApplications as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumRemoteDebugApplications {}
windows_core::imp::define_interface!(IJsDebug, IJsDebug_Vtbl, 0xbe0e89da_2ac5_4c04_ac5e_59956aae3613);
windows_core::imp::interface_hierarchy!(IJsDebug, windows_core::IUnknown);
impl IJsDebug {
    pub unsafe fn OpenVirtualProcess<P2>(&self, processid: u32, runtimejsbaseaddress: u64, pdatatarget: P2) -> windows_core::Result<IJsDebugProcess>
    where
        P2: windows_core::Param<IJsDebugDataTarget>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenVirtualProcess)(windows_core::Interface::as_raw(self), processid, runtimejsbaseaddress, pdatatarget.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenVirtualProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IJsDebug_Impl: windows_core::IUnknownImpl {
    fn OpenVirtualProcess(&self, processid: u32, runtimejsbaseaddress: u64, pdatatarget: windows_core::Ref<'_, IJsDebugDataTarget>) -> windows_core::Result<IJsDebugProcess>;
}
impl IJsDebug_Vtbl {
    pub const fn new<Identity: IJsDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenVirtualProcess<Identity: IJsDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: u32, runtimejsbaseaddress: u64, pdatatarget: *mut core::ffi::c_void, ppprocess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebug_Impl::OpenVirtualProcess(this, core::mem::transmute_copy(&processid), core::mem::transmute_copy(&runtimejsbaseaddress), core::mem::transmute_copy(&pdatatarget)) {
                    Ok(ok__) => {
                        ppprocess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenVirtualProcess: OpenVirtualProcess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebug {}
windows_core::imp::define_interface!(IJsDebugBreakPoint, IJsDebugBreakPoint_Vtbl, 0xdf6773e3_ed8d_488b_8a3e_5812577d1542);
windows_core::imp::interface_hierarchy!(IJsDebugBreakPoint, windows_core::IUnknown);
impl IJsDebugBreakPoint {
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disable(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDocumentPosition(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentPosition)(windows_core::Interface::as_raw(self), pdocumentid as _, pcharacteroffset as _, pstatementcharcount as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugBreakPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IJsDebugBreakPoint_Impl: windows_core::IUnknownImpl {
    fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetDocumentPosition(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::Result<()>;
}
impl IJsDebugBreakPoint_Vtbl {
    pub const fn new<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEnabled<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugBreakPoint_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        pisenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enable<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugBreakPoint_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn Disable<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugBreakPoint_Impl::Disable(this).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugBreakPoint_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GetDocumentPosition<Identity: IJsDebugBreakPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugBreakPoint_Impl::GetDocumentPosition(this, core::mem::transmute_copy(&pdocumentid), core::mem::transmute_copy(&pcharacteroffset), core::mem::transmute_copy(&pstatementcharcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Disable: Disable::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetDocumentPosition: GetDocumentPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugBreakPoint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugBreakPoint {}
windows_core::imp::define_interface!(IJsDebugDataTarget, IJsDebugDataTarget_Vtbl, 0x53b28977_53a1_48e5_9000_5d0dfa893931);
windows_core::imp::interface_hierarchy!(IJsDebugDataTarget, windows_core::IUnknown);
impl IJsDebugDataTarget {
    pub unsafe fn ReadMemory(&self, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: &mut [u8], pbytesread: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadMemory)(windows_core::Interface::as_raw(self), address, flags, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pbytesread as _).ok() }
    }
    pub unsafe fn WriteMemory(&self, address: u64, pmemory: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteMemory)(windows_core::Interface::as_raw(self), address, core::mem::transmute(pmemory.as_ptr()), pmemory.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn AllocateVirtualMemory(&self, address: u64, size: u32, allocationtype: u32, pageprotection: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateVirtualMemory)(windows_core::Interface::as_raw(self), address, size, allocationtype, pageprotection, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeVirtualMemory(&self, address: u64, size: u32, freetype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreeVirtualMemory)(windows_core::Interface::as_raw(self), address, size, freetype).ok() }
    }
    pub unsafe fn GetTlsValue(&self, threadid: u32, tlsindex: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTlsValue)(windows_core::Interface::as_raw(self), threadid, tlsindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReadBSTR(&self, address: u64) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBSTR)(windows_core::Interface::as_raw(self), address, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ReadNullTerminatedString(&self, address: u64, charactersize: u16, maxcharacters: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadNullTerminatedString)(windows_core::Interface::as_raw(self), address, charactersize, maxcharacters, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateStackFrameEnumerator(&self, threadid: u32) -> windows_core::Result<IEnumJsStackFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStackFrameEnumerator)(windows_core::Interface::as_raw(self), threadid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetThreadContext(&self, threadid: u32, contextflags: u32, pcontext: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetThreadContext)(windows_core::Interface::as_raw(self), threadid, contextflags, pcontext.len().try_into().unwrap(), core::mem::transmute(pcontext.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugDataTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64, JsDebugReadMemoryFlags, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub WriteMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const u8, u32) -> windows_core::HRESULT,
    pub AllocateVirtualMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, u32, u32, *mut u64) -> windows_core::HRESULT,
    pub FreeVirtualMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, u32) -> windows_core::HRESULT,
    pub GetTlsValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u64) -> windows_core::HRESULT,
    pub ReadBSTR: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadNullTerminatedString: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u16, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStackFrameEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IJsDebugDataTarget_Impl: windows_core::IUnknownImpl {
    fn ReadMemory(&self, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> windows_core::Result<()>;
    fn WriteMemory(&self, address: u64, pmemory: *const u8, size: u32) -> windows_core::Result<()>;
    fn AllocateVirtualMemory(&self, address: u64, size: u32, allocationtype: u32, pageprotection: u32) -> windows_core::Result<u64>;
    fn FreeVirtualMemory(&self, address: u64, size: u32, freetype: u32) -> windows_core::Result<()>;
    fn GetTlsValue(&self, threadid: u32, tlsindex: u32) -> windows_core::Result<u64>;
    fn ReadBSTR(&self, address: u64) -> windows_core::Result<windows_core::BSTR>;
    fn ReadNullTerminatedString(&self, address: u64, charactersize: u16, maxcharacters: u32) -> windows_core::Result<windows_core::BSTR>;
    fn CreateStackFrameEnumerator(&self, threadid: u32) -> windows_core::Result<IEnumJsStackFrames>;
    fn GetThreadContext(&self, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IJsDebugDataTarget_Vtbl {
    pub const fn new<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMemory<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugDataTarget_Impl::ReadMemory(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pbytesread)).into()
            }
        }
        unsafe extern "system" fn WriteMemory<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, pmemory: *const u8, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugDataTarget_Impl::WriteMemory(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&pmemory), core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn AllocateVirtualMemory<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, size: u32, allocationtype: u32, pageprotection: u32, pallocatedaddress: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugDataTarget_Impl::AllocateVirtualMemory(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&size), core::mem::transmute_copy(&allocationtype), core::mem::transmute_copy(&pageprotection)) {
                    Ok(ok__) => {
                        pallocatedaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeVirtualMemory<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, size: u32, freetype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugDataTarget_Impl::FreeVirtualMemory(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&size), core::mem::transmute_copy(&freetype)).into()
            }
        }
        unsafe extern "system" fn GetTlsValue<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32, tlsindex: u32, pvalue: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugDataTarget_Impl::GetTlsValue(this, core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&tlsindex)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadBSTR<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, pstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugDataTarget_Impl::ReadBSTR(this, core::mem::transmute_copy(&address)) {
                    Ok(ok__) => {
                        pstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadNullTerminatedString<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u64, charactersize: u16, maxcharacters: u32, pstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugDataTarget_Impl::ReadNullTerminatedString(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&charactersize), core::mem::transmute_copy(&maxcharacters)) {
                    Ok(ok__) => {
                        pstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStackFrameEnumerator<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugDataTarget_Impl::CreateStackFrameEnumerator(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        ppenumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThreadContext<Identity: IJsDebugDataTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugDataTarget_Impl::GetThreadContext(this, core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&contextsize), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMemory: ReadMemory::<Identity, OFFSET>,
            WriteMemory: WriteMemory::<Identity, OFFSET>,
            AllocateVirtualMemory: AllocateVirtualMemory::<Identity, OFFSET>,
            FreeVirtualMemory: FreeVirtualMemory::<Identity, OFFSET>,
            GetTlsValue: GetTlsValue::<Identity, OFFSET>,
            ReadBSTR: ReadBSTR::<Identity, OFFSET>,
            ReadNullTerminatedString: ReadNullTerminatedString::<Identity, OFFSET>,
            CreateStackFrameEnumerator: CreateStackFrameEnumerator::<Identity, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugDataTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugDataTarget {}
windows_core::imp::define_interface!(IJsDebugFrame, IJsDebugFrame_Vtbl, 0xc9196637_ab9d_44b2_bad2_13b95b3f390e);
windows_core::imp::interface_hierarchy!(IJsDebugFrame, windows_core::IUnknown);
impl IJsDebugFrame {
    pub unsafe fn GetStackRange(&self, pstart: *mut u64, pend: *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStackRange)(windows_core::Interface::as_raw(self), pstart as _, pend as _).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDocumentPositionWithId(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentPositionWithId)(windows_core::Interface::as_raw(self), pdocumentid as _, pcharacteroffset as _, pstatementcharcount as _).ok() }
    }
    pub unsafe fn GetDocumentPositionWithName(&self, pdocumentname: *mut windows_core::BSTR, pline: *mut u32, pcolumn: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDocumentPositionWithName)(windows_core::Interface::as_raw(self), core::mem::transmute(pdocumentname), pline as _, pcolumn as _).ok() }
    }
    pub unsafe fn GetDebugProperty(&self) -> windows_core::Result<IJsDebugProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDebugProperty)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReturnAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReturnAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Evaluate<P0>(&self, pexpressiontext: P0, ppdebugproperty: *mut Option<IJsDebugProperty>, perror: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Evaluate)(windows_core::Interface::as_raw(self), pexpressiontext.param().abi(), core::mem::transmute(ppdebugproperty), core::mem::transmute(perror)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStackRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentPositionWithId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentPositionWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDebugProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReturnAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Evaluate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IJsDebugFrame_Impl: windows_core::IUnknownImpl {
    fn GetStackRange(&self, pstart: *mut u64, pend: *mut u64) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDocumentPositionWithId(&self, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::Result<()>;
    fn GetDocumentPositionWithName(&self, pdocumentname: *mut windows_core::BSTR, pline: *mut u32, pcolumn: *mut u32) -> windows_core::Result<()>;
    fn GetDebugProperty(&self) -> windows_core::Result<IJsDebugProperty>;
    fn GetReturnAddress(&self) -> windows_core::Result<u64>;
    fn Evaluate(&self, pexpressiontext: &windows_core::PCWSTR, ppdebugproperty: windows_core::OutRef<'_, IJsDebugProperty>, perror: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IJsDebugFrame_Vtbl {
    pub const fn new<Identity: IJsDebugFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStackRange<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut u64, pend: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugFrame_Impl::GetStackRange(this, core::mem::transmute_copy(&pstart), core::mem::transmute_copy(&pend)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugFrame_Impl::GetName(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocumentPositionWithId<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugFrame_Impl::GetDocumentPositionWithId(this, core::mem::transmute_copy(&pdocumentid), core::mem::transmute_copy(&pcharacteroffset), core::mem::transmute_copy(&pstatementcharcount)).into()
            }
        }
        unsafe extern "system" fn GetDocumentPositionWithName<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentname: *mut *mut core::ffi::c_void, pline: *mut u32, pcolumn: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugFrame_Impl::GetDocumentPositionWithName(this, core::mem::transmute_copy(&pdocumentname), core::mem::transmute_copy(&pline), core::mem::transmute_copy(&pcolumn)).into()
            }
        }
        unsafe extern "system" fn GetDebugProperty<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdebugproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugFrame_Impl::GetDebugProperty(this) {
                    Ok(ok__) => {
                        ppdebugproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReturnAddress<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preturnaddress: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugFrame_Impl::GetReturnAddress(this) {
                    Ok(ok__) => {
                        preturnaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Evaluate<Identity: IJsDebugFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpressiontext: windows_core::PCWSTR, ppdebugproperty: *mut *mut core::ffi::c_void, perror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugFrame_Impl::Evaluate(this, core::mem::transmute(&pexpressiontext), core::mem::transmute_copy(&ppdebugproperty), core::mem::transmute_copy(&perror)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStackRange: GetStackRange::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDocumentPositionWithId: GetDocumentPositionWithId::<Identity, OFFSET>,
            GetDocumentPositionWithName: GetDocumentPositionWithName::<Identity, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, OFFSET>,
            GetReturnAddress: GetReturnAddress::<Identity, OFFSET>,
            Evaluate: Evaluate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugFrame as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugFrame {}
windows_core::imp::define_interface!(IJsDebugProcess, IJsDebugProcess_Vtbl, 0x3d587168_6a2d_4041_bd3b_0de674502862);
windows_core::imp::interface_hierarchy!(IJsDebugProcess, windows_core::IUnknown);
impl IJsDebugProcess {
    pub unsafe fn CreateStackWalker(&self, threadid: u32) -> windows_core::Result<IJsDebugStackWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStackWalker)(windows_core::Interface::as_raw(self), threadid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBreakPoint(&self, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: bool) -> windows_core::Result<IJsDebugBreakPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBreakPoint)(windows_core::Interface::as_raw(self), documentid, characteroffset, charactercount, isenabled.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PerformAsyncBreak(&self, threadid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PerformAsyncBreak)(windows_core::Interface::as_raw(self), threadid).ok() }
    }
    pub unsafe fn GetExternalStepAddress(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExternalStepAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugProcess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStackWalker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, u32, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerformAsyncBreak: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExternalStepAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IJsDebugProcess_Impl: windows_core::IUnknownImpl {
    fn CreateStackWalker(&self, threadid: u32) -> windows_core::Result<IJsDebugStackWalker>;
    fn CreateBreakPoint(&self, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: windows_core::BOOL) -> windows_core::Result<IJsDebugBreakPoint>;
    fn PerformAsyncBreak(&self, threadid: u32) -> windows_core::Result<()>;
    fn GetExternalStepAddress(&self) -> windows_core::Result<u64>;
}
impl IJsDebugProcess_Vtbl {
    pub const fn new<Identity: IJsDebugProcess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStackWalker<Identity: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32, ppstackwalker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugProcess_Impl::CreateStackWalker(this, core::mem::transmute_copy(&threadid)) {
                    Ok(ok__) => {
                        ppstackwalker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBreakPoint<Identity: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: windows_core::BOOL, ppdebugbreakpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugProcess_Impl::CreateBreakPoint(this, core::mem::transmute_copy(&documentid), core::mem::transmute_copy(&characteroffset), core::mem::transmute_copy(&charactercount), core::mem::transmute_copy(&isenabled)) {
                    Ok(ok__) => {
                        ppdebugbreakpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PerformAsyncBreak<Identity: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugProcess_Impl::PerformAsyncBreak(this, core::mem::transmute_copy(&threadid)).into()
            }
        }
        unsafe extern "system" fn GetExternalStepAddress<Identity: IJsDebugProcess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcodeaddress: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugProcess_Impl::GetExternalStepAddress(this) {
                    Ok(ok__) => {
                        pcodeaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStackWalker: CreateStackWalker::<Identity, OFFSET>,
            CreateBreakPoint: CreateBreakPoint::<Identity, OFFSET>,
            PerformAsyncBreak: PerformAsyncBreak::<Identity, OFFSET>,
            GetExternalStepAddress: GetExternalStepAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugProcess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugProcess {}
windows_core::imp::define_interface!(IJsDebugProperty, IJsDebugProperty_Vtbl, 0xf8ffcf2b_3aa4_4320_85c3_52a312ba9633);
windows_core::imp::interface_hierarchy!(IJsDebugProperty, windows_core::IUnknown);
impl IJsDebugProperty {
    pub unsafe fn GetPropertyInfo(&self, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), nradix, core::mem::transmute(ppropertyinfo)).ok() }
    }
    pub unsafe fn GetMembers(&self, members: JS_PROPERTY_MEMBERS) -> windows_core::Result<IJsEnumDebugProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMembers)(windows_core::Interface::as_raw(self), members, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut JsDebugPropertyInfo) -> windows_core::HRESULT,
    pub GetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, JS_PROPERTY_MEMBERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IJsDebugProperty_Impl: windows_core::IUnknownImpl {
    fn GetPropertyInfo(&self, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> windows_core::Result<()>;
    fn GetMembers(&self, members: JS_PROPERTY_MEMBERS) -> windows_core::Result<IJsEnumDebugProperty>;
}
impl IJsDebugProperty_Vtbl {
    pub const fn new<Identity: IJsDebugProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyInfo<Identity: IJsDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsDebugProperty_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&ppropertyinfo)).into()
            }
        }
        unsafe extern "system" fn GetMembers<Identity: IJsDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, members: JS_PROPERTY_MEMBERS, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugProperty_Impl::GetMembers(this, core::mem::transmute_copy(&members)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            GetMembers: GetMembers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugProperty as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugProperty {}
windows_core::imp::define_interface!(IJsDebugStackWalker, IJsDebugStackWalker_Vtbl, 0xdb24b094_73c4_456c_a4ec_e90ea00bdfe3);
windows_core::imp::interface_hierarchy!(IJsDebugStackWalker, windows_core::IUnknown);
impl IJsDebugStackWalker {
    pub unsafe fn GetNext(&self) -> windows_core::Result<IJsDebugFrame> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsDebugStackWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IJsDebugStackWalker_Impl: windows_core::IUnknownImpl {
    fn GetNext(&self) -> windows_core::Result<IJsDebugFrame>;
}
impl IJsDebugStackWalker_Vtbl {
    pub const fn new<Identity: IJsDebugStackWalker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNext<Identity: IJsDebugStackWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsDebugStackWalker_Impl::GetNext(this) {
                    Ok(ok__) => {
                        ppframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNext: GetNext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsDebugStackWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsDebugStackWalker {}
windows_core::imp::define_interface!(IJsEnumDebugProperty, IJsEnumDebugProperty_Vtbl, 0x4092432f_2f0f_4fe1_b638_5b74a52cdcbe);
windows_core::imp::interface_hierarchy!(IJsEnumDebugProperty, windows_core::IUnknown);
impl IJsEnumDebugProperty {
    pub unsafe fn Next(&self, ppdebugproperty: &mut [Option<IJsDebugProperty>], pactualcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppdebugproperty.len().try_into().unwrap(), core::mem::transmute(ppdebugproperty.as_ptr()), pactualcount as _).ok() }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsEnumDebugProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IJsEnumDebugProperty_Impl: windows_core::IUnknownImpl {
    fn Next(&self, count: u32, ppdebugproperty: *mut Option<IJsDebugProperty>, pactualcount: *mut u32) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IJsEnumDebugProperty_Vtbl {
    pub const fn new<Identity: IJsEnumDebugProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IJsEnumDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, ppdebugproperty: *mut *mut core::ffi::c_void, pactualcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IJsEnumDebugProperty_Impl::Next(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&ppdebugproperty), core::mem::transmute_copy(&pactualcount)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IJsEnumDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsEnumDebugProperty_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET>, GetCount: GetCount::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsEnumDebugProperty as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IJsEnumDebugProperty {}
windows_core::imp::define_interface!(IMachineDebugManager, IMachineDebugManager_Vtbl, 0x51973c2c_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IMachineDebugManager, windows_core::IUnknown);
impl IMachineDebugManager {
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IRemoteDebugApplication>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveApplication)(windows_core::Interface::as_raw(self), dwappcookie).ok() }
    }
    pub unsafe fn EnumApplications(&self) -> windows_core::Result<IEnumRemoteDebugApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMachineDebugManager_Impl: windows_core::IUnknownImpl {
    fn AddApplication(&self, pda: windows_core::Ref<'_, IRemoteDebugApplication>) -> windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()>;
    fn EnumApplications(&self) -> windows_core::Result<IEnumRemoteDebugApplications>;
}
impl IMachineDebugManager_Vtbl {
    pub const fn new<Identity: IMachineDebugManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddApplication<Identity: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, pdwappcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineDebugManager_Impl::AddApplication(this, core::mem::transmute_copy(&pda)) {
                    Ok(ok__) => {
                        pdwappcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineDebugManager_Impl::RemoveApplication(this, core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        unsafe extern "system" fn EnumApplications<Identity: IMachineDebugManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineDebugManager_Impl::EnumApplications(this) {
                    Ok(ok__) => {
                        ppeda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddApplication: AddApplication::<Identity, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, OFFSET>,
            EnumApplications: EnumApplications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineDebugManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMachineDebugManager {}
windows_core::imp::define_interface!(IMachineDebugManagerCookie, IMachineDebugManagerCookie_Vtbl, 0x51973c2d_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IMachineDebugManagerCookie, windows_core::IUnknown);
impl IMachineDebugManagerCookie {
    pub unsafe fn AddApplication<P0>(&self, pda: P0, dwdebugappcookie: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IRemoteDebugApplication>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), dwdebugappcookie, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveApplication(&self, dwdebugappcookie: u32, dwappcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveApplication)(windows_core::Interface::as_raw(self), dwdebugappcookie, dwappcookie).ok() }
    }
    pub unsafe fn EnumApplications(&self) -> windows_core::Result<IEnumRemoteDebugApplications> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumApplications)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManagerCookie_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub EnumApplications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMachineDebugManagerCookie_Impl: windows_core::IUnknownImpl {
    fn AddApplication(&self, pda: windows_core::Ref<'_, IRemoteDebugApplication>, dwdebugappcookie: u32) -> windows_core::Result<u32>;
    fn RemoveApplication(&self, dwdebugappcookie: u32, dwappcookie: u32) -> windows_core::Result<()>;
    fn EnumApplications(&self) -> windows_core::Result<IEnumRemoteDebugApplications>;
}
impl IMachineDebugManagerCookie_Vtbl {
    pub const fn new<Identity: IMachineDebugManagerCookie_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddApplication<Identity: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, dwdebugappcookie: u32, pdwappcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineDebugManagerCookie_Impl::AddApplication(this, core::mem::transmute_copy(&pda), core::mem::transmute_copy(&dwdebugappcookie)) {
                    Ok(ok__) => {
                        pdwappcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdebugappcookie: u32, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineDebugManagerCookie_Impl::RemoveApplication(this, core::mem::transmute_copy(&dwdebugappcookie), core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        unsafe extern "system" fn EnumApplications<Identity: IMachineDebugManagerCookie_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineDebugManagerCookie_Impl::EnumApplications(this) {
                    Ok(ok__) => {
                        ppeda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddApplication: AddApplication::<Identity, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, OFFSET>,
            EnumApplications: EnumApplications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineDebugManagerCookie as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMachineDebugManagerCookie {}
windows_core::imp::define_interface!(IMachineDebugManagerEvents, IMachineDebugManagerEvents_Vtbl, 0x51973c2e_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IMachineDebugManagerEvents, windows_core::IUnknown);
impl IMachineDebugManagerEvents {
    pub unsafe fn onAddApplication<P0>(&self, pda: P0, dwappcookie: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplication>,
    {
        unsafe { (windows_core::Interface::vtable(self).onAddApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), dwappcookie).ok() }
    }
    pub unsafe fn onRemoveApplication<P0>(&self, pda: P0, dwappcookie: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplication>,
    {
        unsafe { (windows_core::Interface::vtable(self).onRemoveApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), dwappcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineDebugManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub onAddApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub onRemoveApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IMachineDebugManagerEvents_Impl: windows_core::IUnknownImpl {
    fn onAddApplication(&self, pda: windows_core::Ref<'_, IRemoteDebugApplication>, dwappcookie: u32) -> windows_core::Result<()>;
    fn onRemoveApplication(&self, pda: windows_core::Ref<'_, IRemoteDebugApplication>, dwappcookie: u32) -> windows_core::Result<()>;
}
impl IMachineDebugManagerEvents_Vtbl {
    pub const fn new<Identity: IMachineDebugManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn onAddApplication<Identity: IMachineDebugManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineDebugManagerEvents_Impl::onAddApplication(this, core::mem::transmute_copy(&pda), core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        unsafe extern "system" fn onRemoveApplication<Identity: IMachineDebugManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineDebugManagerEvents_Impl::onRemoveApplication(this, core::mem::transmute_copy(&pda), core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            onAddApplication: onAddApplication::<Identity, OFFSET>,
            onRemoveApplication: onRemoveApplication::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineDebugManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMachineDebugManagerEvents {}
windows_core::imp::define_interface!(IProcessDebugManager32, IProcessDebugManager32_Vtbl, 0x51973c2f_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IProcessDebugManager32, windows_core::IUnknown);
impl IProcessDebugManager32 {
    pub unsafe fn CreateApplication(&self) -> windows_core::Result<IDebugApplication32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultApplication(&self) -> windows_core::Result<IDebugApplication32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDebugApplication32>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveApplication)(windows_core::Interface::as_raw(self), dwappcookie).ok() }
    }
    pub unsafe fn CreateDebugDocumentHelper<P0>(&self, punkouter: P0) -> windows_core::Result<IDebugDocumentHelper32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDebugDocumentHelper)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDebugManager32_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateDebugDocumentHelper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProcessDebugManager32_Impl: windows_core::IUnknownImpl {
    fn CreateApplication(&self) -> windows_core::Result<IDebugApplication32>;
    fn GetDefaultApplication(&self) -> windows_core::Result<IDebugApplication32>;
    fn AddApplication(&self, pda: windows_core::Ref<'_, IDebugApplication32>) -> windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()>;
    fn CreateDebugDocumentHelper(&self, punkouter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<IDebugDocumentHelper32>;
}
impl IProcessDebugManager32_Vtbl {
    pub const fn new<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateApplication<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager32_Impl::CreateApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager32_Impl::GetDefaultApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddApplication<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, pdwappcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager32_Impl::AddApplication(this, core::mem::transmute_copy(&pda)) {
                    Ok(ok__) => {
                        pdwappcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessDebugManager32_Impl::RemoveApplication(this, core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: IProcessDebugManager32_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pddh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager32_Impl::CreateDebugDocumentHelper(this, core::mem::transmute_copy(&punkouter)) {
                    Ok(ok__) => {
                        pddh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateApplication: CreateApplication::<Identity, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, OFFSET>,
            AddApplication: AddApplication::<Identity, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessDebugManager32 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessDebugManager32 {}
windows_core::imp::define_interface!(IProcessDebugManager64, IProcessDebugManager64_Vtbl, 0x56b9fc1c_63a9_4cc1_ac21_087d69a17fab);
windows_core::imp::interface_hierarchy!(IProcessDebugManager64, windows_core::IUnknown);
impl IProcessDebugManager64 {
    pub unsafe fn CreateApplication(&self) -> windows_core::Result<IDebugApplication64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultApplication(&self) -> windows_core::Result<IDebugApplication64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddApplication<P0>(&self, pda: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDebugApplication64>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddApplication)(windows_core::Interface::as_raw(self), pda.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveApplication)(windows_core::Interface::as_raw(self), dwappcookie).ok() }
    }
    pub unsafe fn CreateDebugDocumentHelper<P0>(&self, punkouter: P0) -> windows_core::Result<IDebugDocumentHelper64>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDebugDocumentHelper)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessDebugManager64_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveApplication: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateDebugDocumentHelper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProcessDebugManager64_Impl: windows_core::IUnknownImpl {
    fn CreateApplication(&self) -> windows_core::Result<IDebugApplication64>;
    fn GetDefaultApplication(&self) -> windows_core::Result<IDebugApplication64>;
    fn AddApplication(&self, pda: windows_core::Ref<'_, IDebugApplication64>) -> windows_core::Result<u32>;
    fn RemoveApplication(&self, dwappcookie: u32) -> windows_core::Result<()>;
    fn CreateDebugDocumentHelper(&self, punkouter: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<IDebugDocumentHelper64>;
}
impl IProcessDebugManager64_Vtbl {
    pub const fn new<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateApplication<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager64_Impl::CreateApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager64_Impl::GetDefaultApplication(this) {
                    Ok(ok__) => {
                        ppda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddApplication<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pda: *mut core::ffi::c_void, pdwappcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager64_Impl::AddApplication(this, core::mem::transmute_copy(&pda)) {
                    Ok(ok__) => {
                        pdwappcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveApplication<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessDebugManager64_Impl::RemoveApplication(this, core::mem::transmute_copy(&dwappcookie)).into()
            }
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: IProcessDebugManager64_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pddh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessDebugManager64_Impl::CreateDebugDocumentHelper(this, core::mem::transmute_copy(&punkouter)) {
                    Ok(ok__) => {
                        pddh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateApplication: CreateApplication::<Identity, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, OFFSET>,
            AddApplication: AddApplication::<Identity, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessDebugManager64 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessDebugManager64 {}
windows_core::imp::define_interface!(IProvideExpressionContexts, IProvideExpressionContexts_Vtbl, 0x51973c41_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IProvideExpressionContexts, windows_core::IUnknown);
impl IProvideExpressionContexts {
    pub unsafe fn EnumExpressionContexts(&self) -> windows_core::Result<IEnumDebugExpressionContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumExpressionContexts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideExpressionContexts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumExpressionContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IProvideExpressionContexts_Impl: windows_core::IUnknownImpl {
    fn EnumExpressionContexts(&self) -> windows_core::Result<IEnumDebugExpressionContexts>;
}
impl IProvideExpressionContexts_Vtbl {
    pub const fn new<Identity: IProvideExpressionContexts_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumExpressionContexts<Identity: IProvideExpressionContexts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedec: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProvideExpressionContexts_Impl::EnumExpressionContexts(this) {
                    Ok(ok__) => {
                        ppedec.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumExpressionContexts: EnumExpressionContexts::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideExpressionContexts as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProvideExpressionContexts {}
windows_core::imp::define_interface!(IRemoteDebugApplication, IRemoteDebugApplication_Vtbl, 0x51973c30_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IRemoteDebugApplication, windows_core::IUnknown);
impl IRemoteDebugApplication {
    pub unsafe fn ResumeFromBreakPoint<P0>(&self, prptfocus: P0, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResumeFromBreakPoint)(windows_core::Interface::as_raw(self), prptfocus.param().abi(), bra, era).ok() }
    }
    pub unsafe fn CauseBreak(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CauseBreak)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ConnectDebugger<P0>(&self, pad: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IApplicationDebugger>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectDebugger)(windows_core::Interface::as_raw(self), pad.param().abi()).ok() }
    }
    pub unsafe fn DisconnectDebugger(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectDebugger)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDebugger(&self) -> windows_core::Result<IApplicationDebugger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDebugger)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInstanceAtApplication<P1>(&self, rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstanceAtApplication)(windows_core::Interface::as_raw(self), rclsid, punkouter.param().abi(), dwclscontext, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QueryAlive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryAlive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumThreads(&self) -> windows_core::Result<IEnumRemoteDebugApplicationThreads> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumThreads)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRootNode(&self) -> windows_core::Result<IDebugApplicationNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumGlobalExpressionContexts(&self) -> windows_core::Result<IEnumDebugExpressionContexts> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumGlobalExpressionContexts)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplication_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResumeFromBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BREAKRESUMEACTION, ERRORRESUMEACTION) -> windows_core::HRESULT,
    pub CauseBreak: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectDebugger: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceAtApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryAlive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumGlobalExpressionContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteDebugApplication_Impl: windows_core::IUnknownImpl {
    fn ResumeFromBreakPoint(&self, prptfocus: windows_core::Ref<'_, IRemoteDebugApplicationThread>, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> windows_core::Result<()>;
    fn CauseBreak(&self) -> windows_core::Result<()>;
    fn ConnectDebugger(&self, pad: windows_core::Ref<'_, IApplicationDebugger>) -> windows_core::Result<()>;
    fn DisconnectDebugger(&self) -> windows_core::Result<()>;
    fn GetDebugger(&self) -> windows_core::Result<IApplicationDebugger>;
    fn CreateInstanceAtApplication(&self, rclsid: *const windows_core::GUID, punkouter: windows_core::Ref<'_, windows_core::IUnknown>, dwclscontext: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryAlive(&self) -> windows_core::Result<()>;
    fn EnumThreads(&self) -> windows_core::Result<IEnumRemoteDebugApplicationThreads>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRootNode(&self) -> windows_core::Result<IDebugApplicationNode>;
    fn EnumGlobalExpressionContexts(&self) -> windows_core::Result<IEnumDebugExpressionContexts>;
}
impl IRemoteDebugApplication_Vtbl {
    pub const fn new<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResumeFromBreakPoint<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prptfocus: *mut core::ffi::c_void, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication_Impl::ResumeFromBreakPoint(this, core::mem::transmute_copy(&prptfocus), core::mem::transmute_copy(&bra), core::mem::transmute_copy(&era)).into()
            }
        }
        unsafe extern "system" fn CauseBreak<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication_Impl::CauseBreak(this).into()
            }
        }
        unsafe extern "system" fn ConnectDebugger<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pad: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication_Impl::ConnectDebugger(this, core::mem::transmute_copy(&pad)).into()
            }
        }
        unsafe extern "system" fn DisconnectDebugger<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication_Impl::DisconnectDebugger(this).into()
            }
        }
        unsafe extern "system" fn GetDebugger<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pad: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::GetDebugger(this) {
                    Ok(ok__) => {
                        pad.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceAtApplication<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclscontext: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::CreateInstanceAtApplication(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        ppvobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryAlive<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication_Impl::QueryAlive(this).into()
            }
        }
        unsafe extern "system" fn EnumThreads<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperdat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::EnumThreads(this) {
                    Ok(ok__) => {
                        pperdat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetName<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootNode<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdanroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::GetRootNode(this) {
                    Ok(ok__) => {
                        ppdanroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumGlobalExpressionContexts<Identity: IRemoteDebugApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedec: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication_Impl::EnumGlobalExpressionContexts(this) {
                    Ok(ok__) => {
                        ppedec.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResumeFromBreakPoint: ResumeFromBreakPoint::<Identity, OFFSET>,
            CauseBreak: CauseBreak::<Identity, OFFSET>,
            ConnectDebugger: ConnectDebugger::<Identity, OFFSET>,
            DisconnectDebugger: DisconnectDebugger::<Identity, OFFSET>,
            GetDebugger: GetDebugger::<Identity, OFFSET>,
            CreateInstanceAtApplication: CreateInstanceAtApplication::<Identity, OFFSET>,
            QueryAlive: QueryAlive::<Identity, OFFSET>,
            EnumThreads: EnumThreads::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetRootNode: GetRootNode::<Identity, OFFSET>,
            EnumGlobalExpressionContexts: EnumGlobalExpressionContexts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugApplication as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugApplication {}
windows_core::imp::define_interface!(IRemoteDebugApplication110, IRemoteDebugApplication110_Vtbl, 0xd5fe005b_2836_485e_b1f9_89d91aa24fd4);
windows_core::imp::interface_hierarchy!(IRemoteDebugApplication110, windows_core::IUnknown);
impl IRemoteDebugApplication110 {
    pub unsafe fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDebuggerOptions)(windows_core::Interface::as_raw(self), mask, value).ok() }
    }
    pub unsafe fn GetCurrentDebuggerOptions(&self) -> windows_core::Result<SCRIPT_DEBUGGER_OPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentDebuggerOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMainThread(&self) -> windows_core::Result<IRemoteDebugApplicationThread> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMainThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplication110_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDebuggerOptions: unsafe extern "system" fn(*mut core::ffi::c_void, SCRIPT_DEBUGGER_OPTIONS, SCRIPT_DEBUGGER_OPTIONS) -> windows_core::HRESULT,
    pub GetCurrentDebuggerOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCRIPT_DEBUGGER_OPTIONS) -> windows_core::HRESULT,
    pub GetMainThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteDebugApplication110_Impl: windows_core::IUnknownImpl {
    fn SetDebuggerOptions(&self, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> windows_core::Result<()>;
    fn GetCurrentDebuggerOptions(&self) -> windows_core::Result<SCRIPT_DEBUGGER_OPTIONS>;
    fn GetMainThread(&self) -> windows_core::Result<IRemoteDebugApplicationThread>;
}
impl IRemoteDebugApplication110_Vtbl {
    pub const fn new<Identity: IRemoteDebugApplication110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDebuggerOptions<Identity: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplication110_Impl::SetDebuggerOptions(this, core::mem::transmute_copy(&mask), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetCurrentDebuggerOptions<Identity: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcurrentoptions: *mut SCRIPT_DEBUGGER_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication110_Impl::GetCurrentDebuggerOptions(this) {
                    Ok(ok__) => {
                        pcurrentoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMainThread<Identity: IRemoteDebugApplication110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthread: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplication110_Impl::GetMainThread(this) {
                    Ok(ok__) => {
                        ppthread.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDebuggerOptions: SetDebuggerOptions::<Identity, OFFSET>,
            GetCurrentDebuggerOptions: GetCurrentDebuggerOptions::<Identity, OFFSET>,
            GetMainThread: GetMainThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugApplication110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugApplication110 {}
windows_core::imp::define_interface!(IRemoteDebugApplicationEvents, IRemoteDebugApplicationEvents_Vtbl, 0x51973c33_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IRemoteDebugApplicationEvents, windows_core::IUnknown);
impl IRemoteDebugApplicationEvents {
    pub unsafe fn OnConnectDebugger<P0>(&self, pad: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IApplicationDebugger>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnectDebugger)(windows_core::Interface::as_raw(self), pad.param().abi()).ok() }
    }
    pub unsafe fn OnDisconnectDebugger(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDisconnectDebugger)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnSetName<P0>(&self, pstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetName)(windows_core::Interface::as_raw(self), pstrname.param().abi()).ok() }
    }
    pub unsafe fn OnDebugOutput<P0>(&self, pstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDebugOutput)(windows_core::Interface::as_raw(self), pstr.param().abi()).ok() }
    }
    pub unsafe fn OnClose(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnEnterBreakPoint<P0>(&self, prdat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEnterBreakPoint)(windows_core::Interface::as_raw(self), prdat.param().abi()).ok() }
    }
    pub unsafe fn OnLeaveBreakPoint<P0>(&self, prdat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLeaveBreakPoint)(windows_core::Interface::as_raw(self), prdat.param().abi()).ok() }
    }
    pub unsafe fn OnCreateThread<P0>(&self, prdat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCreateThread)(windows_core::Interface::as_raw(self), prdat.param().abi()).ok() }
    }
    pub unsafe fn OnDestroyThread<P0>(&self, prdat: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDestroyThread)(windows_core::Interface::as_raw(self), prdat.param().abi()).ok() }
    }
    pub unsafe fn OnBreakFlagChange<P1>(&self, abf: u32, prdatsteppingthread: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IRemoteDebugApplicationThread>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBreakFlagChange)(windows_core::Interface::as_raw(self), abf, prdatsteppingthread.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplicationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectDebugger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDisconnectDebugger: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnSetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnEnterBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLeaveBreakPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnCreateThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDestroyThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBreakFlagChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteDebugApplicationEvents_Impl: windows_core::IUnknownImpl {
    fn OnConnectDebugger(&self, pad: windows_core::Ref<'_, IApplicationDebugger>) -> windows_core::Result<()>;
    fn OnDisconnectDebugger(&self) -> windows_core::Result<()>;
    fn OnSetName(&self, pstrname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDebugOutput(&self, pstr: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
    fn OnEnterBreakPoint(&self, prdat: windows_core::Ref<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn OnLeaveBreakPoint(&self, prdat: windows_core::Ref<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn OnCreateThread(&self, prdat: windows_core::Ref<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn OnDestroyThread(&self, prdat: windows_core::Ref<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
    fn OnBreakFlagChange(&self, abf: u32, prdatsteppingthread: windows_core::Ref<'_, IRemoteDebugApplicationThread>) -> windows_core::Result<()>;
}
impl IRemoteDebugApplicationEvents_Vtbl {
    pub const fn new<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnectDebugger<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pad: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnConnectDebugger(this, core::mem::transmute_copy(&pad)).into()
            }
        }
        unsafe extern "system" fn OnDisconnectDebugger<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnDisconnectDebugger(this).into()
            }
        }
        unsafe extern "system" fn OnSetName<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnSetName(this, core::mem::transmute(&pstrname)).into()
            }
        }
        unsafe extern "system" fn OnDebugOutput<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstr: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnDebugOutput(this, core::mem::transmute(&pstr)).into()
            }
        }
        unsafe extern "system" fn OnClose<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnClose(this).into()
            }
        }
        unsafe extern "system" fn OnEnterBreakPoint<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnEnterBreakPoint(this, core::mem::transmute_copy(&prdat)).into()
            }
        }
        unsafe extern "system" fn OnLeaveBreakPoint<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnLeaveBreakPoint(this, core::mem::transmute_copy(&prdat)).into()
            }
        }
        unsafe extern "system" fn OnCreateThread<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnCreateThread(this, core::mem::transmute_copy(&prdat)).into()
            }
        }
        unsafe extern "system" fn OnDestroyThread<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnDestroyThread(this, core::mem::transmute_copy(&prdat)).into()
            }
        }
        unsafe extern "system" fn OnBreakFlagChange<Identity: IRemoteDebugApplicationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, abf: u32, prdatsteppingthread: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationEvents_Impl::OnBreakFlagChange(this, core::mem::transmute_copy(&abf), core::mem::transmute_copy(&prdatsteppingthread)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnectDebugger: OnConnectDebugger::<Identity, OFFSET>,
            OnDisconnectDebugger: OnDisconnectDebugger::<Identity, OFFSET>,
            OnSetName: OnSetName::<Identity, OFFSET>,
            OnDebugOutput: OnDebugOutput::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
            OnEnterBreakPoint: OnEnterBreakPoint::<Identity, OFFSET>,
            OnLeaveBreakPoint: OnLeaveBreakPoint::<Identity, OFFSET>,
            OnCreateThread: OnCreateThread::<Identity, OFFSET>,
            OnDestroyThread: OnDestroyThread::<Identity, OFFSET>,
            OnBreakFlagChange: OnBreakFlagChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugApplicationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugApplicationEvents {}
windows_core::imp::define_interface!(IRemoteDebugApplicationThread, IRemoteDebugApplicationThread_Vtbl, 0x51973c37_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IRemoteDebugApplicationThread, windows_core::IUnknown);
impl IRemoteDebugApplicationThread {
    pub unsafe fn GetSystemThreadId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetApplication(&self) -> windows_core::Result<IRemoteDebugApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumStackFrames(&self) -> windows_core::Result<IEnumDebugStackFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumStackFrames)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut windows_core::BSTR, pbstrstate: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdescription), core::mem::transmute(pbstrstate)).ok() }
    }
    pub unsafe fn SetNextStatement<P0, P1>(&self, pstackframe: P0, pcodecontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDebugStackFrame>,
        P1: windows_core::Param<IDebugCodeContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNextStatement)(windows_core::Interface::as_raw(self), pstackframe.param().abi(), pcodecontext.param().abi()).ok() }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSuspendCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuspendCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugApplicationThread_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumStackFrames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNextStatement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSuspendCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IRemoteDebugApplicationThread_Impl: windows_core::IUnknownImpl {
    fn GetSystemThreadId(&self) -> windows_core::Result<u32>;
    fn GetApplication(&self) -> windows_core::Result<IRemoteDebugApplication>;
    fn EnumStackFrames(&self) -> windows_core::Result<IEnumDebugStackFrames>;
    fn GetDescription(&self, pbstrdescription: *mut windows_core::BSTR, pbstrstate: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetNextStatement(&self, pstackframe: windows_core::Ref<'_, IDebugStackFrame>, pcodecontext: windows_core::Ref<'_, IDebugCodeContext>) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<u32>;
    fn Suspend(&self) -> windows_core::Result<u32>;
    fn Resume(&self) -> windows_core::Result<u32>;
    fn GetSuspendCount(&self) -> windows_core::Result<u32>;
}
impl IRemoteDebugApplicationThread_Vtbl {
    pub const fn new<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemThreadId<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::GetSystemThreadId(this) {
                    Ok(ok__) => {
                        dwthreadid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplication<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprda: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::GetApplication(this) {
                    Ok(ok__) => {
                        pprda.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumStackFrames<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppedsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::EnumStackFrames(this) {
                    Ok(ok__) => {
                        ppedsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void, pbstrstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationThread_Impl::GetDescription(this, core::mem::transmute_copy(&pbstrdescription), core::mem::transmute_copy(&pbstrstate)).into()
            }
        }
        unsafe extern "system" fn SetNextStatement<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstackframe: *mut core::ffi::c_void, pcodecontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugApplicationThread_Impl::SetNextStatement(this, core::mem::transmute_copy(&pstackframe), core::mem::transmute_copy(&pcodecontext)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::GetState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Suspend<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::Suspend(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resume<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::Resume(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSuspendCount<Identity: IRemoteDebugApplicationThread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDebugApplicationThread_Impl::GetSuspendCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemThreadId: GetSystemThreadId::<Identity, OFFSET>,
            GetApplication: GetApplication::<Identity, OFFSET>,
            EnumStackFrames: EnumStackFrames::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetNextStatement: SetNextStatement::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            GetSuspendCount: GetSuspendCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugApplicationThread as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugApplicationThread {}
windows_core::imp::define_interface!(IRemoteDebugCriticalErrorEvent110, IRemoteDebugCriticalErrorEvent110_Vtbl, 0x2f69c611_6b14_47e8_9260_4bb7c52f504b);
windows_core::imp::interface_hierarchy!(IRemoteDebugCriticalErrorEvent110, windows_core::IUnknown);
impl IRemoteDebugCriticalErrorEvent110 {
    pub unsafe fn GetErrorInfo(&self, pbstrsource: *mut windows_core::BSTR, pmessageid: *mut i32, pbstrmessage: *mut windows_core::BSTR, pplocation: *mut Option<IDebugDocumentContext>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrsource), pmessageid as _, core::mem::transmute(pbstrmessage), core::mem::transmute(pplocation)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugCriticalErrorEvent110_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteDebugCriticalErrorEvent110_Impl: windows_core::IUnknownImpl {
    fn GetErrorInfo(&self, pbstrsource: *mut windows_core::BSTR, pmessageid: *mut i32, pbstrmessage: *mut windows_core::BSTR, pplocation: windows_core::OutRef<'_, IDebugDocumentContext>) -> windows_core::Result<()>;
}
impl IRemoteDebugCriticalErrorEvent110_Vtbl {
    pub const fn new<Identity: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorInfo<Identity: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void, pmessageid: *mut i32, pbstrmessage: *mut *mut core::ffi::c_void, pplocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugCriticalErrorEvent110_Impl::GetErrorInfo(this, core::mem::transmute_copy(&pbstrsource), core::mem::transmute_copy(&pmessageid), core::mem::transmute_copy(&pbstrmessage), core::mem::transmute_copy(&pplocation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugCriticalErrorEvent110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugCriticalErrorEvent110 {}
windows_core::imp::define_interface!(IRemoteDebugInfoEvent110, IRemoteDebugInfoEvent110_Vtbl, 0x9ff56bb6_eb89_4c0f_8823_cc2a4c0b7f26);
windows_core::imp::interface_hierarchy!(IRemoteDebugInfoEvent110, windows_core::IUnknown);
impl IRemoteDebugInfoEvent110 {
    pub unsafe fn GetEventInfo(&self, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut windows_core::BSTR, pbstrurl: *mut windows_core::BSTR, pplocation: *mut Option<IDebugDocumentContext>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEventInfo)(windows_core::Interface::as_raw(self), pmessagetype as _, core::mem::transmute(pbstrmessage), core::mem::transmute(pbstrurl), core::mem::transmute(pplocation)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDebugInfoEvent110_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEBUG_EVENT_INFO_TYPE, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteDebugInfoEvent110_Impl: windows_core::IUnknownImpl {
    fn GetEventInfo(&self, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut windows_core::BSTR, pbstrurl: *mut windows_core::BSTR, pplocation: windows_core::OutRef<'_, IDebugDocumentContext>) -> windows_core::Result<()>;
}
impl IRemoteDebugInfoEvent110_Vtbl {
    pub const fn new<Identity: IRemoteDebugInfoEvent110_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventInfo<Identity: IRemoteDebugInfoEvent110_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut *mut core::ffi::c_void, pbstrurl: *mut *mut core::ffi::c_void, pplocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDebugInfoEvent110_Impl::GetEventInfo(this, core::mem::transmute_copy(&pmessagetype), core::mem::transmute_copy(&pbstrmessage), core::mem::transmute_copy(&pbstrurl), core::mem::transmute_copy(&pplocation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetEventInfo: GetEventInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDebugInfoEvent110 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteDebugInfoEvent110 {}
windows_core::imp::define_interface!(IScriptEntry, IScriptEntry_Vtbl, 0x0aee2a95_bcbb_11d0_8c72_00c04fc2b085);
impl core::ops::Deref for IScriptEntry {
    type Target = IScriptNode;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScriptEntry, windows_core::IUnknown, IScriptNode);
impl IScriptEntry {
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetText<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    pub unsafe fn GetBody(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBody)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBody<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBody)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    pub unsafe fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetItemName<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetItemName)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSignature(&self, ppti: *mut Option<super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), core::mem::transmute(ppti), pimethod as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignature<P0>(&self, pti: P0, imethod: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Com::ITypeInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSignature)(windows_core::Interface::as_raw(self), pti.param().abi(), imethod).ok() }
    }
    pub unsafe fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRange)(windows_core::Interface::as_raw(self), pichmin as _, pcch as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptEntry_Vtbl {
    pub base__: IScriptNode_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBody: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetItemName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSignature: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSignature: usize,
    pub GetRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptEntry_Impl: IScriptNode_Impl {
    fn GetText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBody(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBody(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetItemName(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSignature(&self, ppti: windows_core::OutRef<'_, super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> windows_core::Result<()>;
    fn SetSignature(&self, pti: windows_core::Ref<'_, super::super::super::Com::ITypeInfo>, imethod: u32) -> windows_core::Result<()>;
    fn GetRange(&self, pichmin: *mut u32, pcch: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IScriptEntry_Vtbl {
    pub const fn new<Identity: IScriptEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptEntry_Impl::GetText(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetText<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::SetText(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetBody<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptEntry_Impl::GetBody(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBody<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::SetBody(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptEntry_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::SetName(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetItemName<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptEntry_Impl::GetItemName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetItemName<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::SetItemName(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppti: *mut *mut core::ffi::c_void, pimethod: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::GetSignature(this, core::mem::transmute_copy(&ppti), core::mem::transmute_copy(&pimethod)).into()
            }
        }
        unsafe extern "system" fn SetSignature<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pti: *mut core::ffi::c_void, imethod: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::SetSignature(this, core::mem::transmute_copy(&pti), core::mem::transmute_copy(&imethod)).into()
            }
        }
        unsafe extern "system" fn GetRange<Identity: IScriptEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pichmin: *mut u32, pcch: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptEntry_Impl::GetRange(this, core::mem::transmute_copy(&pichmin), core::mem::transmute_copy(&pcch)).into()
            }
        }
        Self {
            base__: IScriptNode_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            GetBody: GetBody::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            GetItemName: GetItemName::<Identity, OFFSET>,
            SetItemName: SetItemName::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScriptEntry as windows_core::Interface>::IID || iid == &<IScriptNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IScriptEntry {}
windows_core::imp::define_interface!(IScriptInvocationContext, IScriptInvocationContext_Vtbl, 0x5d7741b7_af7e_4a2a_85e5_c77f4d0659fb);
windows_core::imp::interface_hierarchy!(IScriptInvocationContext, windows_core::IUnknown);
impl IScriptInvocationContext {
    pub unsafe fn GetContextType(&self) -> windows_core::Result<SCRIPT_INVOCATION_CONTEXT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetContextObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContextObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptInvocationContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContextType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SCRIPT_INVOCATION_CONTEXT_TYPE) -> windows_core::HRESULT,
    pub GetContextDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContextObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IScriptInvocationContext_Impl: windows_core::IUnknownImpl {
    fn GetContextType(&self) -> windows_core::Result<SCRIPT_INVOCATION_CONTEXT_TYPE>;
    fn GetContextDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetContextObject(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl IScriptInvocationContext_Vtbl {
    pub const fn new<Identity: IScriptInvocationContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContextType<Identity: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinvocationcontexttype: *mut SCRIPT_INVOCATION_CONTEXT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptInvocationContext_Impl::GetContextType(this) {
                    Ok(ok__) => {
                        pinvocationcontexttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextDescription<Identity: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptInvocationContext_Impl::GetContextDescription(this) {
                    Ok(ok__) => {
                        pdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetContextObject<Identity: IScriptInvocationContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontextobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptInvocationContext_Impl::GetContextObject(this) {
                    Ok(ok__) => {
                        ppcontextobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContextType: GetContextType::<Identity, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, OFFSET>,
            GetContextObject: GetContextObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScriptInvocationContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScriptInvocationContext {}
windows_core::imp::define_interface!(IScriptNode, IScriptNode_Vtbl, 0x0aee2a94_bcbb_11d0_8c72_00c04fc2b085);
windows_core::imp::interface_hierarchy!(IScriptNode, windows_core::IUnknown);
impl IScriptNode {
    pub unsafe fn Alive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Alive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<IScriptNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIndexInParent(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndexInParent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCookie(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNumberOfChildren(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChild(&self, isn: u32) -> windows_core::Result<IScriptNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), isn, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreateChildEntry<P2>(&self, isn: u32, dwcookie: u32, pszdelimiter: P2) -> windows_core::Result<IScriptEntry>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildEntry)(windows_core::Interface::as_raw(self), isn, dwcookie, pszdelimiter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateChildHandler<P0, P3, P4, P5>(&self, pszdefaultname: P0, prgpsznames: &[windows_core::PCWSTR], pszevent: P3, pszdelimiter: P4, ptisignature: P5, imethodsignature: u32, isn: u32, dwcookie: u32) -> windows_core::Result<IScriptEntry>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<super::super::super::Com::ITypeInfo>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildHandler)(windows_core::Interface::as_raw(self), pszdefaultname.param().abi(), core::mem::transmute(prgpsznames.as_ptr()), prgpsznames.len().try_into().unwrap(), pszevent.param().abi(), pszdelimiter.param().abi(), ptisignature.param().abi(), imethodsignature, isn, dwcookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptNode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndexInParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNumberOfChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateChildEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateChildHandler: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateChildHandler: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptNode_Impl: windows_core::IUnknownImpl {
    fn Alive(&self) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetParent(&self) -> windows_core::Result<IScriptNode>;
    fn GetIndexInParent(&self) -> windows_core::Result<u32>;
    fn GetCookie(&self) -> windows_core::Result<u32>;
    fn GetNumberOfChildren(&self) -> windows_core::Result<u32>;
    fn GetChild(&self, isn: u32) -> windows_core::Result<IScriptNode>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateChildEntry(&self, isn: u32, dwcookie: u32, pszdelimiter: &windows_core::PCWSTR) -> windows_core::Result<IScriptEntry>;
    fn CreateChildHandler(&self, pszdefaultname: &windows_core::PCWSTR, prgpsznames: *const windows_core::PCWSTR, cpsznames: u32, pszevent: &windows_core::PCWSTR, pszdelimiter: &windows_core::PCWSTR, ptisignature: windows_core::Ref<'_, super::super::super::Com::ITypeInfo>, imethodsignature: u32, isn: u32, dwcookie: u32) -> windows_core::Result<IScriptEntry>;
}
#[cfg(feature = "Win32_System_Com")]
impl IScriptNode_Vtbl {
    pub const fn new<Identity: IScriptNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alive<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptNode_Impl::Alive(this).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptNode_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsnparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetParent(this) {
                    Ok(ok__) => {
                        ppsnparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIndexInParent<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisn: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetIndexInParent(this) {
                    Ok(ok__) => {
                        pisn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCookie<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetCookie(this) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfChildren<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsn: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetNumberOfChildren(this) {
                    Ok(ok__) => {
                        pcsn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChild<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isn: u32, ppsn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetChild(this, core::mem::transmute_copy(&isn)) {
                    Ok(ok__) => {
                        ppsn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateChildEntry<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isn: u32, dwcookie: u32, pszdelimiter: windows_core::PCWSTR, ppse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::CreateChildEntry(this, core::mem::transmute_copy(&isn), core::mem::transmute_copy(&dwcookie), core::mem::transmute(&pszdelimiter)) {
                    Ok(ok__) => {
                        ppse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateChildHandler<Identity: IScriptNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultname: windows_core::PCWSTR, prgpsznames: *const windows_core::PCWSTR, cpsznames: u32, pszevent: windows_core::PCWSTR, pszdelimiter: windows_core::PCWSTR, ptisignature: *mut core::ffi::c_void, imethodsignature: u32, isn: u32, dwcookie: u32, ppse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptNode_Impl::CreateChildHandler(this, core::mem::transmute(&pszdefaultname), core::mem::transmute_copy(&prgpsznames), core::mem::transmute_copy(&cpsznames), core::mem::transmute(&pszevent), core::mem::transmute(&pszdelimiter), core::mem::transmute_copy(&ptisignature), core::mem::transmute_copy(&imethodsignature), core::mem::transmute_copy(&isn), core::mem::transmute_copy(&dwcookie)) {
                    Ok(ok__) => {
                        ppse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alive: Alive::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
            GetIndexInParent: GetIndexInParent::<Identity, OFFSET>,
            GetCookie: GetCookie::<Identity, OFFSET>,
            GetNumberOfChildren: GetNumberOfChildren::<Identity, OFFSET>,
            GetChild: GetChild::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            CreateChildEntry: CreateChildEntry::<Identity, OFFSET>,
            CreateChildHandler: CreateChildHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScriptNode as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IScriptNode {}
windows_core::imp::define_interface!(IScriptScriptlet, IScriptScriptlet_Vtbl, 0x0aee2a96_bcbb_11d0_8c72_00c04fc2b085);
impl core::ops::Deref for IScriptScriptlet {
    type Target = IScriptEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScriptScriptlet, windows_core::IUnknown, IScriptNode, IScriptEntry);
impl IScriptScriptlet {
    pub unsafe fn GetSubItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubItemName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubItemName<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSubItemName)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    pub unsafe fn GetEventName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEventName<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventName)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
    pub unsafe fn GetSimpleEventName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSimpleEventName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSimpleEventName<P0>(&self, psz: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSimpleEventName)(windows_core::Interface::as_raw(self), psz.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptScriptlet_Vtbl {
    pub base__: IScriptEntry_Vtbl,
    pub GetSubItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubItemName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetEventName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSimpleEventName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSimpleEventName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptScriptlet_Impl: IScriptEntry_Impl {
    fn GetSubItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubItemName(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetEventName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventName(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSimpleEventName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSimpleEventName(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IScriptScriptlet_Vtbl {
    pub const fn new<Identity: IScriptScriptlet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSubItemName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptScriptlet_Impl::GetSubItemName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubItemName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptScriptlet_Impl::SetSubItemName(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetEventName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptScriptlet_Impl::GetEventName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptScriptlet_Impl::SetEventName(this, core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn GetSimpleEventName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScriptScriptlet_Impl::GetSimpleEventName(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSimpleEventName<Identity: IScriptScriptlet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScriptScriptlet_Impl::SetSimpleEventName(this, core::mem::transmute(&psz)).into()
            }
        }
        Self {
            base__: IScriptEntry_Vtbl::new::<Identity, OFFSET>(),
            GetSubItemName: GetSubItemName::<Identity, OFFSET>,
            SetSubItemName: SetSubItemName::<Identity, OFFSET>,
            GetEventName: GetEventName::<Identity, OFFSET>,
            SetEventName: SetEventName::<Identity, OFFSET>,
            GetSimpleEventName: GetSimpleEventName::<Identity, OFFSET>,
            SetSimpleEventName: SetSimpleEventName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScriptScriptlet as windows_core::Interface>::IID || iid == &<IScriptNode as windows_core::Interface>::IID || iid == &<IScriptEntry as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IScriptScriptlet {}
windows_core::imp::define_interface!(ISimpleConnectionPoint, ISimpleConnectionPoint_Vtbl, 0x51973c3e_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(ISimpleConnectionPoint, windows_core::IUnknown);
impl ISimpleConnectionPoint {
    pub unsafe fn GetEventCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DescribeEvents(&self, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut windows_core::BSTR, pceventsfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DescribeEvents)(windows_core::Interface::as_raw(self), ievent, cevents, prgid as _, core::mem::transmute(prgbstr), pceventsfetched as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, pdisp: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::super::Com::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pdisp.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleConnectionPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEventCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DescribeEvents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut i32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISimpleConnectionPoint_Impl: windows_core::IUnknownImpl {
    fn GetEventCount(&self) -> windows_core::Result<u32>;
    fn DescribeEvents(&self, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut windows_core::BSTR, pceventsfetched: *mut u32) -> windows_core::Result<()>;
    fn Advise(&self, pdisp: windows_core::Ref<'_, super::super::super::Com::IDispatch>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISimpleConnectionPoint_Vtbl {
    pub const fn new<Identity: ISimpleConnectionPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEventCount<Identity: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleConnectionPoint_Impl::GetEventCount(this) {
                    Ok(ok__) => {
                        pulcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DescribeEvents<Identity: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut *mut core::ffi::c_void, pceventsfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleConnectionPoint_Impl::DescribeEvents(this, core::mem::transmute_copy(&ievent), core::mem::transmute_copy(&cevents), core::mem::transmute_copy(&prgid), core::mem::transmute_copy(&prgbstr), core::mem::transmute_copy(&pceventsfetched)).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisp: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleConnectionPoint_Impl::Advise(this, core::mem::transmute_copy(&pdisp)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ISimpleConnectionPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleConnectionPoint_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEventCount: GetEventCount::<Identity, OFFSET>,
            DescribeEvents: DescribeEvents::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleConnectionPoint as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISimpleConnectionPoint {}
windows_core::imp::define_interface!(ITridentEventSink, ITridentEventSink_Vtbl, 0x1dc9ca50_06ef_11d2_8415_006008c3fbfc);
windows_core::imp::interface_hierarchy!(ITridentEventSink, windows_core::IUnknown);
impl ITridentEventSink {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn FireEvent<P0>(&self, pstrevent: P0, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FireEvent)(windows_core::Interface::as_raw(self), pstrevent.param().abi(), pdp, core::mem::transmute(pvarres), core::mem::transmute(pei)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub FireEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::super::super::Com::DISPPARAMS, *mut super::super::super::Variant::VARIANT, *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    FireEvent: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITridentEventSink_Impl: windows_core::IUnknownImpl {
    fn FireEvent(&self, pstrevent: &windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITridentEventSink_Vtbl {
    pub const fn new<Identity: ITridentEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FireEvent<Identity: ITridentEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrevent: windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITridentEventSink_Impl::FireEvent(this, core::mem::transmute(&pstrevent), core::mem::transmute_copy(&pdp), core::mem::transmute_copy(&pvarres), core::mem::transmute_copy(&pei)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FireEvent: FireEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITridentEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITridentEventSink {}
windows_core::imp::define_interface!(IWebAppDiagnosticsObjectInitialization, IWebAppDiagnosticsObjectInitialization_Vtbl, 0x16ff3a42_a5f5_432b_b625_8e8e16f57e15);
windows_core::imp::interface_hierarchy!(IWebAppDiagnosticsObjectInitialization, windows_core::IUnknown);
impl IWebAppDiagnosticsObjectInitialization {
    pub unsafe fn Initialize<P1>(&self, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hpassedhandle, pdebugapplication.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAppDiagnosticsObjectInitialization_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::HANDLE_PTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWebAppDiagnosticsObjectInitialization_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IWebAppDiagnosticsObjectInitialization_Vtbl {
    pub const fn new<Identity: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAppDiagnosticsObjectInitialization_Impl::Initialize(this, core::mem::transmute_copy(&hpassedhandle), core::mem::transmute_copy(&pdebugapplication)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAppDiagnosticsObjectInitialization as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWebAppDiagnosticsObjectInitialization {}
windows_core::imp::define_interface!(IWebAppDiagnosticsSetup, IWebAppDiagnosticsSetup_Vtbl, 0x379bfbe1_c6c9_432a_93e1_6d17656c538c);
windows_core::imp::interface_hierarchy!(IWebAppDiagnosticsSetup, windows_core::IUnknown);
impl IWebAppDiagnosticsSetup {
    pub unsafe fn DiagnosticsSupported(&self) -> windows_core::Result<super::super::super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiagnosticsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateObjectWithSiteAtWebApp(&self, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, hpasstoobject: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectWithSiteAtWebApp)(windows_core::Interface::as_raw(self), rclsid, dwclscontext, riid, hpasstoobject).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAppDiagnosticsSetup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DiagnosticsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CreateObjectWithSiteAtWebApp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, usize) -> windows_core::HRESULT,
}
pub trait IWebAppDiagnosticsSetup_Impl: windows_core::IUnknownImpl {
    fn DiagnosticsSupported(&self) -> windows_core::Result<super::super::super::super::Foundation::VARIANT_BOOL>;
    fn CreateObjectWithSiteAtWebApp(&self, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, hpasstoobject: usize) -> windows_core::Result<()>;
}
impl IWebAppDiagnosticsSetup_Vtbl {
    pub const fn new<Identity: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DiagnosticsSupported<Identity: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebAppDiagnosticsSetup_Impl::DiagnosticsSupported(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateObjectWithSiteAtWebApp<Identity: IWebAppDiagnosticsSetup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclscontext: u32, riid: *const windows_core::GUID, hpasstoobject: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAppDiagnosticsSetup_Impl::CreateObjectWithSiteAtWebApp(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&hpasstoobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DiagnosticsSupported: DiagnosticsSupported::<Identity, OFFSET>,
            CreateObjectWithSiteAtWebApp: CreateObjectWithSiteAtWebApp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAppDiagnosticsSetup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWebAppDiagnosticsSetup {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JS_NATIVE_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JS_PROPERTY_ATTRIBUTES(pub i32);
pub const JS_PROPERTY_ATTRIBUTE_NONE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(0i32);
pub const JS_PROPERTY_FAKE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(2i32);
pub const JS_PROPERTY_FRAME_INCATCHBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(64i32);
pub const JS_PROPERTY_FRAME_INFINALLYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(128i32);
pub const JS_PROPERTY_FRAME_INTRYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(32i32);
pub const JS_PROPERTY_HAS_CHILDREN: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JS_PROPERTY_MEMBERS(pub i32);
pub const JS_PROPERTY_MEMBERS_ALL: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(0i32);
pub const JS_PROPERTY_MEMBERS_ARGUMENTS: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(1i32);
pub const JS_PROPERTY_METHOD: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(4i32);
pub const JS_PROPERTY_NATIVE_WINRT_POINTER: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(16i32);
pub const JS_PROPERTY_READONLY: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(8i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JsDebugPropertyInfo {
    pub name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub r#type: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub value: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub fullName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub attr: JS_PROPERTY_ATTRIBUTES,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsDebugReadMemoryFlags(pub i32);
impl JsDebugReadMemoryFlags {
    pub const None: Self = Self(0i32);
    pub const JsDebugAllowPartialRead: Self = Self(1i32);
}
pub const MachineDebugManager_DEBUG: windows_core::GUID = windows_core::GUID::from_u128(0x49769cec_3a55_4bb0_b697_88fede77e8ea);
pub const MachineDebugManager_RETAIL: windows_core::GUID = windows_core::GUID::from_u128(0x0c0a3666_30c9_11d0_8f20_00805f2cd064);
pub const OID_JSSIP: windows_core::GUID = windows_core::GUID::from_u128(0x06c9e010_38ce_11d4_a2a3_00104bd35090);
pub const OID_VBSSIP: windows_core::GUID = windows_core::GUID::from_u128(0x1629f04e_2799_4db5_8fe5_ace10f17ebab);
pub const OID_WSFSIP: windows_core::GUID = windows_core::GUID::from_u128(0x1a610570_38ce_11d4_a2a3_00104bd35090);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_EVENT_MASK(pub i32);
impl PROFILER_EVENT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROFILER_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROFILER_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROFILER_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROFILER_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROFILER_EVENT_MASK_TRACE_ALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(3i32);
pub const PROFILER_EVENT_MASK_TRACE_ALL_WITH_DOM: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(7i32);
pub const PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(4i32);
pub const PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(2i32);
pub const PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_HEAP_ENUM_FLAGS(pub i32);
impl PROFILER_HEAP_ENUM_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROFILER_HEAP_ENUM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROFILER_HEAP_ENUM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROFILER_HEAP_ENUM_FLAGS_NONE: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(0i32);
pub const PROFILER_HEAP_ENUM_FLAGS_RELATIONSHIP_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(3i32);
pub const PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(1i32);
pub const PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROFILER_HEAP_OBJECT {
    pub size: u32,
    pub Anonymous: PROFILER_HEAP_OBJECT_0,
    pub typeNameId: u32,
    pub flags: u32,
    pub unused: u16,
    pub optionalInfoCount: u16,
}
impl Default for PROFILER_HEAP_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROFILER_HEAP_OBJECT_0 {
    pub objectId: usize,
    pub externalObjectAddress: *mut core::ffi::c_void,
}
impl Default for PROFILER_HEAP_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_FLAGS(pub i32);
impl PROFILER_HEAP_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROFILER_HEAP_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(8i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_DISPATCH: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(32i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_UNKNOWN: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(16i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_IS_ROOT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_OBJECT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_STATE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(256i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SITE_CLOSED: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_APPROXIMATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(64i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(128i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_DELEGATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2048i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_INSTANCE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(512i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_NAMESPACE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4096i32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_RUNTIMECLASS: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1024i32);
pub const PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    pub infoType: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE,
    pub Anonymous: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0,
}
impl Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    pub prototype: usize,
    pub functionName: windows_core::PCWSTR,
    pub elementAttributesSize: u32,
    pub elementTextChildrenSize: u32,
    pub scopeList: *mut PROFILER_HEAP_OBJECT_SCOPE_LIST,
    pub internalProperty: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP,
    pub namePropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub indexPropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub relationshipList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub eventList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub weakMapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub mapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub setCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
}
impl Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_ATTRIBUTES_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(7i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_TEXT_CHILDREN_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(8i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_FUNCTION_NAME: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(2i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INDEX_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(6i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INTERNAL_PROPERTY: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(4i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(12i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAX_VALUE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_NAME_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(5i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_PROTOTYPE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(1i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_RELATIONSHIPS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(9i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SCOPE_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(3i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(pub i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WEAKMAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(11i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WINRTEVENTS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(10i32);
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP {
    pub relationshipId: u32,
    pub relationshipInfo: PROFILER_RELATIONSHIP_INFO,
    pub Anonymous: PROFILER_HEAP_OBJECT_RELATIONSHIP_0,
}
impl Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    pub numberValue: f64,
    pub stringValue: windows_core::PCWSTR,
    pub bstrValue: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub objectId: usize,
    pub externalObjectAddress: *mut core::ffi::c_void,
    pub subString: *mut PROFILER_PROPERTY_TYPE_SUBSTRING_INFO,
}
impl Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(pub i32);
impl PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_CONST_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(524288i32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_GET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(65536i32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_SET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(131072i32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_LET_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(262144i32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_NONE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(0i32);
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    pub count: u32,
    pub elements: [PROFILER_HEAP_OBJECT_RELATIONSHIP; 1],
}
impl Clone for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_SCOPE_LIST {
    pub count: u32,
    pub scopes: [usize; 1],
}
impl Default for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROFILER_HEAP_SUMMARY {
    pub version: PROFILER_HEAP_SUMMARY_VERSION,
    pub totalHeapSize: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_HEAP_SUMMARY_VERSION(pub i32);
pub const PROFILER_HEAP_SUMMARY_VERSION_1: PROFILER_HEAP_SUMMARY_VERSION = PROFILER_HEAP_SUMMARY_VERSION(1i32);
pub const PROFILER_PROPERTY_TYPE_BSTR: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(5i32);
pub const PROFILER_PROPERTY_TYPE_EXTERNAL_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(4i32);
pub const PROFILER_PROPERTY_TYPE_HEAP_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(3i32);
pub const PROFILER_PROPERTY_TYPE_NUMBER: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(1i32);
pub const PROFILER_PROPERTY_TYPE_STRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(2i32);
pub const PROFILER_PROPERTY_TYPE_SUBSTRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    pub length: u32,
    pub value: windows_core::PCWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_RELATIONSHIP_INFO(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROFILER_SCRIPT_TYPE(pub i32);
pub const PROFILER_SCRIPT_TYPE_DOM: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(3i32);
pub const PROFILER_SCRIPT_TYPE_DYNAMIC: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(1i32);
pub const PROFILER_SCRIPT_TYPE_NATIVE: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(2i32);
pub const PROFILER_SCRIPT_TYPE_USER: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(0i32);
pub const ProcessDebugManager: windows_core::GUID = windows_core::GUID::from_u128(0x78a51822_51f4_11d0_8f20_00805f2cd064);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTGCTYPE(pub i32);
pub const SCRIPTGCTYPE_EXHAUSTIVE: SCRIPTGCTYPE = SCRIPTGCTYPE(1i32);
pub const SCRIPTGCTYPE_NORMAL: SCRIPTGCTYPE = SCRIPTGCTYPE(0i32);
pub const SCRIPTINFO_ITYPEINFO: u32 = 2u32;
pub const SCRIPTINFO_IUNKNOWN: u32 = 1u32;
pub const SCRIPTINTERRUPT_DEBUG: u32 = 1u32;
pub const SCRIPTINTERRUPT_RAISEEXCEPTION: u32 = 2u32;
pub const SCRIPTITEM_CODEONLY: u32 = 512u32;
pub const SCRIPTITEM_GLOBALMEMBERS: u32 = 8u32;
pub const SCRIPTITEM_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTITEM_ISSOURCE: u32 = 4u32;
pub const SCRIPTITEM_ISVISIBLE: u32 = 2u32;
pub const SCRIPTITEM_NOCODE: u32 = 1024u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTLANGUAGEVERSION(pub i32);
pub const SCRIPTLANGUAGEVERSION_5_7: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(1i32);
pub const SCRIPTLANGUAGEVERSION_5_8: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(2i32);
pub const SCRIPTLANGUAGEVERSION_DEFAULT: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(0i32);
pub const SCRIPTLANGUAGEVERSION_MAX: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(255i32);
pub const SCRIPTPROC_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTPROC_IMPLICIT_PARENTS: u32 = 512u32;
pub const SCRIPTPROC_IMPLICIT_THIS: u32 = 256u32;
pub const SCRIPTPROC_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTPROC_ISXDOMAIN: u32 = 1024u32;
pub const SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION: u32 = 1879048194u32;
pub const SCRIPTPROP_BUILDNUMBER: u32 = 3u32;
pub const SCRIPTPROP_CATCHEXCEPTION: u32 = 4097u32;
pub const SCRIPTPROP_CONVERSIONLCID: u32 = 4098u32;
pub const SCRIPTPROP_DEBUGGER: u32 = 4352u32;
pub const SCRIPTPROP_DELAYEDEVENTSINKING: u32 = 4096u32;
pub const SCRIPTPROP_GCCONTROLSOFTCLOSE: u32 = 8192u32;
pub const SCRIPTPROP_HACK_FIBERSUPPORT: u32 = 1879048192u32;
pub const SCRIPTPROP_HACK_TRIDENTEVENTSINK: u32 = 1879048193u32;
pub const SCRIPTPROP_HOSTKEEPALIVE: u32 = 1879048196u32;
pub const SCRIPTPROP_HOSTSTACKREQUIRED: u32 = 4099u32;
pub const SCRIPTPROP_INTEGERMODE: u32 = 12288u32;
pub const SCRIPTPROP_INVOKEVERSIONING: u32 = 16384u32;
pub const SCRIPTPROP_JITDEBUG: u32 = 4353u32;
pub const SCRIPTPROP_MAJORVERSION: u32 = 1u32;
pub const SCRIPTPROP_MINORVERSION: u32 = 2u32;
pub const SCRIPTPROP_NAME: u32 = 0u32;
pub const SCRIPTPROP_SCRIPTSAREFULLYTRUSTED: u32 = 4100u32;
pub const SCRIPTPROP_STRINGCOMPAREINSTANCE: u32 = 12289u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTSTATE(pub i32);
pub const SCRIPTSTATE_CLOSED: SCRIPTSTATE = SCRIPTSTATE(4i32);
pub const SCRIPTSTATE_CONNECTED: SCRIPTSTATE = SCRIPTSTATE(2i32);
pub const SCRIPTSTATE_DISCONNECTED: SCRIPTSTATE = SCRIPTSTATE(3i32);
pub const SCRIPTSTATE_INITIALIZED: SCRIPTSTATE = SCRIPTSTATE(5i32);
pub const SCRIPTSTATE_STARTED: SCRIPTSTATE = SCRIPTSTATE(1i32);
pub const SCRIPTSTATE_UNINITIALIZED: SCRIPTSTATE = SCRIPTSTATE(0i32);
pub const SCRIPTSTAT_INSTRUCTION_COUNT: u32 = 2u32;
pub const SCRIPTSTAT_INTSTRUCTION_TIME: u32 = 3u32;
pub const SCRIPTSTAT_STATEMENT_COUNT: u32 = 1u32;
pub const SCRIPTSTAT_TOTAL_TIME: u32 = 4u32;
pub const SCRIPTTEXT_DELAYEXECUTION: u32 = 1u32;
pub const SCRIPTTEXT_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTTEXT_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTTEXT_ISNONUSERCODE: u32 = 512u32;
pub const SCRIPTTEXT_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTTEXT_ISVISIBLE: u32 = 2u32;
pub const SCRIPTTEXT_ISXDOMAIN: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTTHREADSTATE(pub i32);
pub const SCRIPTTHREADSTATE_NOTINSCRIPT: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(0i32);
pub const SCRIPTTHREADSTATE_RUNNING: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTTRACEINFO(pub i32);
pub const SCRIPTTRACEINFO_COMCALLEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(3i32);
pub const SCRIPTTRACEINFO_COMCALLSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(2i32);
pub const SCRIPTTRACEINFO_CREATEOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(5i32);
pub const SCRIPTTRACEINFO_CREATEOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(4i32);
pub const SCRIPTTRACEINFO_GETOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(7i32);
pub const SCRIPTTRACEINFO_GETOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(6i32);
pub const SCRIPTTRACEINFO_SCRIPTEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(1i32);
pub const SCRIPTTRACEINFO_SCRIPTSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(0i32);
pub const SCRIPTTYPELIB_ISCONTROL: u32 = 16u32;
pub const SCRIPTTYPELIB_ISPERSISTENT: u32 = 64u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTUICHANDLING(pub i32);
pub const SCRIPTUICHANDLING_ALLOW: SCRIPTUICHANDLING = SCRIPTUICHANDLING(0i32);
pub const SCRIPTUICHANDLING_NOUIDEFAULT: SCRIPTUICHANDLING = SCRIPTUICHANDLING(2i32);
pub const SCRIPTUICHANDLING_NOUIERROR: SCRIPTUICHANDLING = SCRIPTUICHANDLING(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPTUICITEM(pub i32);
pub const SCRIPTUICITEM_INPUTBOX: SCRIPTUICITEM = SCRIPTUICITEM(1i32);
pub const SCRIPTUICITEM_MSGBOX: SCRIPTUICITEM = SCRIPTUICITEM(2i32);
pub const SCRIPT_CMPL_COMMIT: u32 = 4u32;
pub const SCRIPT_CMPL_ENUMLIST: u32 = 2u32;
pub const SCRIPT_CMPL_ENUM_TRIGGER: u32 = 1u32;
pub const SCRIPT_CMPL_GLOBALLIST: u32 = 8u32;
pub const SCRIPT_CMPL_MEMBERLIST: u32 = 1u32;
pub const SCRIPT_CMPL_MEMBER_TRIGGER: u32 = 2u32;
pub const SCRIPT_CMPL_NOLIST: u32 = 0u32;
pub const SCRIPT_CMPL_PARAMTIP: u32 = 4u32;
pub const SCRIPT_CMPL_PARAM_TRIGGER: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_DEBUGGER_OPTIONS(pub i32);
impl SCRIPT_DEBUGGER_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SCRIPT_DEBUGGER_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SCRIPT_DEBUGGER_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SCRIPT_DEBUGGER_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SCRIPT_ENCODE_DEFAULT_LANGUAGE: u32 = 1u32;
pub const SCRIPT_ENCODE_NO_ASP_LANGUAGE: u32 = 2u32;
pub const SCRIPT_ENCODE_SECTION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(pub i32);
pub const SCRIPT_E_PROPAGATE: i32 = -2147352318i32;
pub const SCRIPT_E_RECORDED: i32 = -2040119292i32;
pub const SCRIPT_E_REPORTED: i32 = -2147352319i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCRIPT_INVOCATION_CONTEXT_TYPE(pub i32);
pub const SDO_ENABLE_FIRST_CHANCE_EXCEPTIONS: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(1i32);
pub const SDO_ENABLE_LIBRARY_STACK_FRAME: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(8i32);
pub const SDO_ENABLE_NONUSER_CODE_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(4i32);
pub const SDO_ENABLE_WEB_WORKER_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(2i32);
pub const SDO_NONE: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(0i32);
pub const SICT_Event: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(0i32);
pub const SICT_MutationObserverCheckpoint: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(6i32);
pub const SICT_RequestAnimationFrame: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(4i32);
pub const SICT_SetImmediate: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(3i32);
pub const SICT_SetInterval: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(2i32);
pub const SICT_SetTimeout: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(1i32);
pub const SICT_ToString: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(5i32);
pub const SICT_WWAExecAtPriority: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(8i32);
pub const SICT_WWAExecUnsafeLocalFunction: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(7i32);
pub const SOURCETEXT_ATTR_COMMENT: u32 = 2u32;
pub const SOURCETEXT_ATTR_FUNCTION_START: u32 = 64u32;
pub const SOURCETEXT_ATTR_HUMANTEXT: u32 = 32768u32;
pub const SOURCETEXT_ATTR_IDENTIFIER: u32 = 256u32;
pub const SOURCETEXT_ATTR_KEYWORD: u32 = 1u32;
pub const SOURCETEXT_ATTR_MEMBERLOOKUP: u32 = 512u32;
pub const SOURCETEXT_ATTR_NONSOURCE: u32 = 4u32;
pub const SOURCETEXT_ATTR_NUMBER: u32 = 16u32;
pub const SOURCETEXT_ATTR_OPERATOR: u32 = 8u32;
pub const SOURCETEXT_ATTR_STRING: u32 = 32u32;
pub const SOURCETEXT_ATTR_THIS: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TEXT_DOCUMENT_ARRAY {
    pub dwCount: u32,
    pub Members: *mut Option<IDebugDocumentText>,
}
impl Default for TEXT_DOCUMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TEXT_DOC_ATTR_READONLY: u32 = 1u32;
pub const TEXT_DOC_ATTR_TYPE_PRIMARY: u32 = 2u32;
pub const TEXT_DOC_ATTR_TYPE_SCRIPT: u32 = 8u32;
pub const TEXT_DOC_ATTR_TYPE_WORKER: u32 = 4u32;
pub const THREAD_BLOCKED: u32 = 4u32;
pub const THREAD_OUT_OF_CONTEXT: u32 = 8u32;
pub const THREAD_STATE_RUNNING: u32 = 1u32;
pub const THREAD_STATE_SUSPENDED: u32 = 2u32;
pub const fasaCaseSensitive: u32 = 4u32;
pub const fasaPreferInternalHandler: u32 = 1u32;
pub const fasaSupportInternalHandler: u32 = 2u32;
