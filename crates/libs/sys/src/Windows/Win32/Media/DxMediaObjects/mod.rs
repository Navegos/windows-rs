windows_targets::link!("msdmo.dll" "system" fn DMOEnum(guidcategory : *const windows_sys::core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE, ppenum : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn DMOGetName(clsiddmo : *const windows_sys::core::GUID, szname : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn DMOGetTypes(clsiddmo : *const windows_sys::core::GUID, ulinputtypesrequested : u32, pulinputtypessupplied : *mut u32, pinputtypes : *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested : u32, puloutputtypessupplied : *mut u32, poutputtypes : *mut DMO_PARTIAL_MEDIATYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn DMORegister(szname : windows_sys::core::PCWSTR, clsiddmo : *const windows_sys::core::GUID, guidcategory : *const windows_sys::core::GUID, dwflags : u32, cintypes : u32, pintypes : *const DMO_PARTIAL_MEDIATYPE, couttypes : u32, pouttypes : *const DMO_PARTIAL_MEDIATYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn DMOUnregister(clsiddmo : *const windows_sys::core::GUID, guidcategory : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoCopyMediaType(pmtdest : *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoCreateMediaType(ppmt : *mut *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoDeleteMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoDuplicateMediaType(ppmtdest : *mut *mut DMO_MEDIA_TYPE, pmtsrc : *const DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoFreeMediaType(pmt : *mut DMO_MEDIA_TYPE) -> windows_sys::core::HRESULT);
windows_targets::link!("msdmo.dll" "system" fn MoInitMediaType(pmt : *mut DMO_MEDIA_TYPE, cbformat : u32) -> windows_sys::core::HRESULT);
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AGC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe88c9ba0_c557_11d0_8a2b_00a0c9255ac1);
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf665aaba_3e09_4920_aa5f_219811148f09);
pub const DMOCATEGORY_AUDIO_DECODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57f2db8b_e6bb_4513_9d43_dcd2a6593125);
pub const DMOCATEGORY_AUDIO_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3602b3f_0592_48df_a4cd_674721e7ebeb);
pub const DMOCATEGORY_AUDIO_ENCODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33d9a761_90c8_11d0_bd43_00a0c911ce86);
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const DMOCATEGORY_VIDEO_DECODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a69b442_28be_4991_969c_b500adf5d8a8);
pub const DMOCATEGORY_VIDEO_EFFECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd990ee14_776c_4723_be46_3da2f56f10b9);
pub const DMOCATEGORY_VIDEO_ENCODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33d9a760_90c8_11d0_bd43_00a0c911ce86);
pub const DMO_ENUMF_INCLUDE_KEYED: DMO_ENUM_FLAGS = 1i32;
pub type DMO_ENUM_FLAGS = i32;
pub const DMO_E_INVALIDSTREAMINDEX: windows_sys::core::HRESULT = 0x80040201_u32 as _;
pub const DMO_E_INVALIDTYPE: windows_sys::core::HRESULT = 0x80040202_u32 as _;
pub const DMO_E_NOTACCEPTING: windows_sys::core::HRESULT = 0x80040204_u32 as _;
pub const DMO_E_NO_MORE_ITEMS: windows_sys::core::HRESULT = 0x80040206_u32 as _;
pub const DMO_E_TYPE_NOT_ACCEPTED: windows_sys::core::HRESULT = 0x80040205_u32 as _;
pub const DMO_E_TYPE_NOT_SET: windows_sys::core::HRESULT = 0x80040203_u32 as _;
pub const DMO_INPLACE_NORMAL: _DMO_INPLACE_PROCESS_FLAGS = 0i32;
pub const DMO_INPLACE_ZERO: _DMO_INPLACE_PROCESS_FLAGS = 1i32;
pub const DMO_INPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_INPUT_DATA_BUFFER_FLAGS = 8i32;
pub const DMO_INPUT_DATA_BUFFERF_SYNCPOINT: _DMO_INPUT_DATA_BUFFER_FLAGS = 1i32;
pub const DMO_INPUT_DATA_BUFFERF_TIME: _DMO_INPUT_DATA_BUFFER_FLAGS = 2i32;
pub const DMO_INPUT_DATA_BUFFERF_TIMELENGTH: _DMO_INPUT_DATA_BUFFER_FLAGS = 4i32;
pub const DMO_INPUT_STATUSF_ACCEPT_DATA: _DMO_INPUT_STATUS_FLAGS = 1i32;
pub const DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_INPUT_STREAM_INFO_FLAGS = 4i32;
pub const DMO_INPUT_STREAMF_HOLDS_BUFFERS: _DMO_INPUT_STREAM_INFO_FLAGS = 8i32;
pub const DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_INPUT_STREAM_INFO_FLAGS = 2i32;
pub const DMO_INPUT_STREAMF_WHOLE_SAMPLES: _DMO_INPUT_STREAM_INFO_FLAGS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMO_MEDIA_TYPE {
    pub majortype: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMO_OUTPUT_DATA_BUFFER {
    pub pBuffer: *mut core::ffi::c_void,
    pub dwStatus: u32,
    pub rtTimestamp: i64,
    pub rtTimelength: i64,
}
impl Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 8i32;
pub const DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 16777216i32;
pub const DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 1i32;
pub const DMO_OUTPUT_DATA_BUFFERF_TIME: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 2i32;
pub const DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH: _DMO_OUTPUT_DATA_BUFFER_FLAGS = 4i32;
pub const DMO_OUTPUT_STREAMF_DISCARDABLE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 8i32;
pub const DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE: _DMO_OUTPUT_STREAM_INFO_FLAGS = 4i32;
pub const DMO_OUTPUT_STREAMF_OPTIONAL: _DMO_OUTPUT_STREAM_INFO_FLAGS = 16i32;
pub const DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER: _DMO_OUTPUT_STREAM_INFO_FLAGS = 2i32;
pub const DMO_OUTPUT_STREAMF_WHOLE_SAMPLES: _DMO_OUTPUT_STREAM_INFO_FLAGS = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DMO_PARTIAL_MEDIATYPE {
    pub r#type: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
}
pub const DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER: _DMO_PROCESS_OUTPUT_FLAGS = 1i32;
pub const DMO_QUALITY_STATUS_ENABLED: _DMO_QUALITY_STATUS_FLAGS = 1i32;
pub const DMO_REGISTERF_IS_KEYED: DMO_REGISTER_FLAGS = 1i32;
pub type DMO_REGISTER_FLAGS = i32;
pub const DMO_SET_TYPEF_CLEAR: _DMO_SET_TYPE_FLAGS = 2i32;
pub const DMO_SET_TYPEF_TEST_ONLY: _DMO_SET_TYPE_FLAGS = 1i32;
pub const DMO_VOSF_NEEDS_PREVIOUS_SAMPLE: _DMO_VIDEO_OUTPUT_STREAM_FLAGS = 1i32;
pub type _DMO_INPLACE_PROCESS_FLAGS = i32;
pub type _DMO_INPUT_DATA_BUFFER_FLAGS = i32;
pub type _DMO_INPUT_STATUS_FLAGS = i32;
pub type _DMO_INPUT_STREAM_INFO_FLAGS = i32;
pub type _DMO_OUTPUT_DATA_BUFFER_FLAGS = i32;
pub type _DMO_OUTPUT_STREAM_INFO_FLAGS = i32;
pub type _DMO_PROCESS_OUTPUT_FLAGS = i32;
pub type _DMO_QUALITY_STATUS_FLAGS = i32;
pub type _DMO_SET_TYPE_FLAGS = i32;
pub type _DMO_VIDEO_OUTPUT_STREAM_FLAGS = i32;
