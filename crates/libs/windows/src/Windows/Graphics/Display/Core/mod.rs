#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
impl windows_core::TypeKind for HdmiDisplayColorSpace {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HdmiDisplayColorSpace {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayColorSpace;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HdmiDisplayHdr2086Metadata {
    pub RedPrimaryX: u16,
    pub RedPrimaryY: u16,
    pub GreenPrimaryX: u16,
    pub GreenPrimaryY: u16,
    pub BluePrimaryX: u16,
    pub BluePrimaryY: u16,
    pub WhitePointX: u16,
    pub WhitePointY: u16,
    pub MaxMasteringLuminance: u16,
    pub MinMasteringLuminance: u16,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl windows_core::TypeKind for HdmiDisplayHdr2086Metadata {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HdmiDisplayHdr2086Metadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl windows_core::TypeKind for HdmiDisplayHdrOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HdmiDisplayHdrOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayHdrOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdmiDisplayInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HdmiDisplayInformation, windows_core::IUnknown, windows_core::IInspectable);
impl HdmiDisplayInformation {
    pub fn GetSupportedDisplayModes(&self) -> windows_core::Result<windows_collections::IVectorView<HdmiDisplayMode>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedDisplayModes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentDisplayMode(&self) -> windows_core::Result<HdmiDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDisplayMode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultDisplayModeAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDefaultDisplayModeAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestSetCurrentDisplayModeAsync<P0>(&self, mode: P0) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<HdmiDisplayMode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeAsync)(windows_core::Interface::as_raw(this), mode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestSetCurrentDisplayModeWithHdrAsync<P0>(&self, mode: P0, hdroption: HdmiDisplayHdrOption) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<HdmiDisplayMode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAsync)(windows_core::Interface::as_raw(this), mode.param().abi(), hdroption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync<P0>(&self, mode: P0, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata) -> windows_core::Result<windows_future::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<HdmiDisplayMode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(windows_core::Interface::as_raw(this), mode.param().abi(), hdroption, hdrmetadata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayModesChanged<P0>(&self, value: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayModesChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisplayModesChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisplayModesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<HdmiDisplayInformation> {
        Self::IHdmiDisplayInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IHdmiDisplayInformationStatics<R, F: FnOnce(&IHdmiDisplayInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HdmiDisplayInformation, IHdmiDisplayInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HdmiDisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHdmiDisplayInformation>();
}
unsafe impl windows_core::Interface for HdmiDisplayInformation {
    type Vtable = <IHdmiDisplayInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHdmiDisplayInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayInformation";
}
unsafe impl Send for HdmiDisplayInformation {}
unsafe impl Sync for HdmiDisplayInformation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdmiDisplayMode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HdmiDisplayMode, windows_core::IUnknown, windows_core::IInspectable);
impl HdmiDisplayMode {
    pub fn ResolutionWidthInRawPixels(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolutionWidthInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResolutionHeightInRawPixels(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolutionHeightInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RefreshRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RefreshRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StereoEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitsPerPixel(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitsPerPixel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, mode: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HdmiDisplayMode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), mode.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn ColorSpace(&self) -> windows_core::Result<HdmiDisplayColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorSpace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelEncoding(&self) -> windows_core::Result<HdmiDisplayPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSdrLuminanceSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSdrLuminanceSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSmpte2084Supported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSmpte2084Supported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Is2086MetadataSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Is2086MetadataSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDolbyVisionLowLatencySupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IHdmiDisplayMode2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDolbyVisionLowLatencySupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HdmiDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHdmiDisplayMode>();
}
unsafe impl windows_core::Interface for HdmiDisplayMode {
    type Vtable = <IHdmiDisplayMode as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHdmiDisplayMode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayMode";
}
unsafe impl Send for HdmiDisplayMode {}
unsafe impl Sync for HdmiDisplayMode {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
impl windows_core::TypeKind for HdmiDisplayPixelEncoding {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HdmiDisplayPixelEncoding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding;i4)");
}
windows_core::imp::define_interface!(IHdmiDisplayInformation, IHdmiDisplayInformation_Vtbl, 0x130b3c0a_f565_476e_abd5_ea05aee74c69);
impl windows_core::RuntimeType for IHdmiDisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetSupportedDisplayModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultDisplayModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeWithHdrAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HdmiDisplayHdrOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, HdmiDisplayHdrOption, HdmiDisplayHdr2086Metadata, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayModesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDisplayModesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHdmiDisplayInformationStatics, IHdmiDisplayInformationStatics_Vtbl, 0x6ce6b260_f42a_4a15_914c_7b8e2a5a65df);
impl windows_core::RuntimeType for IHdmiDisplayInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHdmiDisplayMode, IHdmiDisplayMode_Vtbl, 0x0c06d5ad_1b90_4f51_9981_ef5a1c0ddf66);
impl windows_core::RuntimeType for IHdmiDisplayMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResolutionWidthInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ResolutionHeightInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HdmiDisplayColorSpace) -> windows_core::HRESULT,
    pub PixelEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HdmiDisplayPixelEncoding) -> windows_core::HRESULT,
    pub IsSdrLuminanceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSmpte2084Supported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Is2086MetadataSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHdmiDisplayMode2, IHdmiDisplayMode2_Vtbl, 0x07cd4e9f_4b3c_42b8_84e7_895368718af2);
impl windows_core::RuntimeType for IHdmiDisplayMode2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDolbyVisionLowLatencySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
