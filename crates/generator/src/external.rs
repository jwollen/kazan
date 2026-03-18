use crate::cdecl::{CBaseType, CType};

/// Shorthand: build a `CType::Base` from a C type name.
const fn ctype(base: &str) -> CType<'_> {
    CType::Base(CBaseType {
        struct_tag: false,
        name: base,
    })
}

fn c_ptr_type(pointee: CType<'static>, is_const: bool) -> CType<'static> {
    CType::Ptr {
        implicit_for_decay: false,
        is_const,
        pointee: Box::new(pointee),
    }
}

/// Well-known C type shorthands used by [`external_types`].
impl CType<'_> {
    pub(crate) const UINT16_T: CType<'static> = ctype("uint16_t");
    pub(crate) const UINT32_T: CType<'static> = ctype("uint32_t");
    pub(crate) const ISIZE_T: CType<'static> = ctype("isize_t");
    pub(crate) const INT: CType<'static> = ctype("int");
    pub(crate) const UINT: CType<'static> = ctype("unsigned int");
    pub(crate) const ULONG: CType<'static> = ctype("unsigned long");
    const HANDLE: CType<'static> = ctype("HANDLE");
}

pub type ExternalTypes = Vec<(&'static str, CType<'static>)>;

/// Canonical list of platform / external types referenced by vk.xml.
pub fn external_types() -> ExternalTypes {
    [
        // X11 / Xlib
        ("VisualID", CType::UINT),
        ("Display", CType::VOID),
        ("Window", CType::ULONG),
        ("RROutput", CType::ULONG),
        // Wayland
        ("wl_display", CType::VOID),
        ("wl_surface", CType::VOID),
        // Win32
        ("HANDLE", CType::ISIZE_T),
        ("HINSTANCE", CType::HANDLE),
        ("HWND", CType::HANDLE),
        ("HMONITOR", CType::HANDLE),
        ("DWORD", CType::ULONG),
        ("LPCWSTR", c_ptr_type(CType::UINT16_T, true)),
        ("SECURITY_ATTRIBUTES", CType::VOID),
        // XCB
        ("xcb_connection_t", CType::VOID),
        ("xcb_window_t", CType::UINT32_T),
        ("xcb_visualid_t", CType::UINT32_T),
        // DirectFB
        ("IDirectFB", CType::VOID),
        ("IDirectFBSurface", CType::VOID),
        // Fuchsia
        ("zx_handle_t", CType::UINT32_T),
        // Google Games Platform
        ("GgpStreamDescriptor", CType::INT),
        ("GgpFrameToken", CType::INT),
        // BlackBerry QNX Screen
        ("_screen_buffer", CType::VOID),
        ("_screen_context", CType::VOID),
        ("_screen_window", CType::VOID),
        // Samsung UBM
        ("ubm_device", CType::VOID),
        ("ubm_surface", CType::VOID),
        // NVIDIA SCI
        ("NvSciSyncAttrList", c_ptr_type(CType::VOID, true)),
        ("NvSciSyncObj", c_ptr_type(CType::VOID, true)),
        ("NvSciSyncFence", c_ptr_type(CType::VOID, false)),
        ("NvSciBufAttrList", c_ptr_type(CType::VOID, true)),
        ("NvSciBufObj", c_ptr_type(CType::VOID, true)),
    ]
    .into_iter()
    .collect()
}
