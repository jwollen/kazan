//! Interop between kazan and [`raw_window_handle`].
//!
//! Provides [`InstanceFn`](crate::window::InstanceFn) to load platform-specific surface creation
//! function pointers and create [`vk::SurfaceKHR`] handles, and
//! [`required_extensions()`](crate::window::required_extensions) to query which instance extensions
//! are needed.

use core::ffi::CStr;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::{InstanceExtensionSet, LoadInstanceFn as _, vk};

/// Loaded platform-specific surface creation function pointers.
///
/// Constructed via [`InstanceFn::load()`], which eagerly loads all surface extensions
/// valid on the current platform. Individual extensions that are not enabled will be
/// `None`.
pub struct InstanceFn {
    #[cfg(target_os = "windows")]
    win32: Option<vk::khr::win32_surface::InstanceFn>,

    #[cfg(all(
        not(target_env = "ohos"),
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ),
    ))]
    wayland: Option<vk::khr::wayland_surface::InstanceFn>,

    #[cfg(all(
        not(target_env = "ohos"),
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ),
    ))]
    xlib: Option<vk::khr::xlib_surface::InstanceFn>,

    #[cfg(all(
        not(target_env = "ohos"),
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ),
    ))]
    xcb: Option<vk::khr::xcb_surface::InstanceFn>,

    #[cfg(target_os = "android")]
    android: Option<vk::khr::android_surface::InstanceFn>,

    #[cfg(target_env = "ohos")]
    ohos: Option<vk::ohos::surface::InstanceFn>,

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    metal: Option<vk::ext::metal_surface::InstanceFn>,
}

impl InstanceFn {
    /// Load all platform-valid surface extensions for the given instance.
    ///
    /// Extensions that are not enabled on the instance will be stored as `None`;
    /// this is not an error. The appropriate extension is selected later based
    /// on the display/window handles passed to [`create_surface()`](Self::create_surface).
    ///
    /// # Safety
    ///
    /// `instance` must be a valid Vulkan instance.
    pub unsafe fn load(entry: &crate::Entry, instance: vk::Instance) -> Self {
        unsafe {
            Self::load_with(|name| {
                core::mem::transmute((entry.static_fn.get_instance_proc_addr)(
                    instance,
                    name.as_ptr(),
                ))
            })
        }
    }

    /// Load all platform-valid surface extensions using a raw function pointer loader.
    ///
    /// # Safety
    ///
    /// The `load` function must return valid function pointers for the given instance.
    pub unsafe fn load_with(load: impl Fn(&CStr) -> Option<vk::PFN_vkVoidFunction>) -> Self {
        Self {
            #[cfg(target_os = "windows")]
            win32: unsafe { vk::khr::win32_surface::InstanceFn::load_with(&load).ok() },

            #[cfg(all(
                not(target_env = "ohos"),
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ),
            ))]
            wayland: unsafe { vk::khr::wayland_surface::InstanceFn::load_with(&load).ok() },

            #[cfg(all(
                not(target_env = "ohos"),
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ),
            ))]
            xlib: unsafe { vk::khr::xlib_surface::InstanceFn::load_with(&load).ok() },

            #[cfg(all(
                not(target_env = "ohos"),
                any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ),
            ))]
            xcb: unsafe { vk::khr::xcb_surface::InstanceFn::load_with(&load).ok() },

            #[cfg(target_os = "android")]
            android: unsafe { vk::khr::android_surface::InstanceFn::load_with(&load).ok() },

            #[cfg(target_env = "ohos")]
            ohos: unsafe { vk::ohos::surface::InstanceFn::load_with(&load).ok() },

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            metal: unsafe { vk::ext::metal_surface::InstanceFn::load_with(&load).ok() },
        }
    }

    /// Query whether the given queue family on a physical device supports presentation
    /// to the display system associated with the given handles.
    ///
    /// Returns `false` if the appropriate surface extension is not loaded or the
    /// display/window handle is not recognized. Returns `true` on platforms that have
    /// no dedicated presentation support query (Android, OHOS, Metal), or when the
    /// Xlib/Xcb window handle has no `visual_id`.
    ///
    /// # Safety
    ///
    /// - `physical_device` must be a valid physical device.
    /// - `display_handle` and `window_handle` must be valid.
    pub unsafe fn get_physical_device_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> bool {
        unsafe {
            match (display_handle, window_handle) {
                #[cfg(target_os = "windows")]
                (RawDisplayHandle::Windows(_), _) if self.win32.is_some() => self
                    .win32
                    .as_ref()
                    .unwrap()
                    .get_physical_device_win32_presentation_support(
                        physical_device,
                        queue_family_index,
                    ),

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Wayland(display), _) if self.wayland.is_some() => self
                    .wayland
                    .as_ref()
                    .unwrap()
                    .get_physical_device_wayland_presentation_support(
                        physical_device,
                        queue_family_index,
                        display.display.as_ptr().cast(),
                    ),

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window))
                    if self.xlib.is_some() =>
                {
                    let Some(dpy) = display.display else {
                        return true;
                    };
                    if window.visual_id == 0 {
                        return true;
                    }
                    self.xlib
                        .as_ref()
                        .unwrap()
                        .get_physical_device_xlib_presentation_support(
                            physical_device,
                            queue_family_index,
                            dpy.as_ptr().cast(),
                            window.visual_id as crate::VisualID,
                        )
                }

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window))
                    if self.xcb.is_some() =>
                {
                    let Some(connection) = display.connection else {
                        return true;
                    };
                    let Some(visual_id) = window.visual_id else {
                        return true;
                    };
                    self.xcb
                        .as_ref()
                        .unwrap()
                        .get_physical_device_xcb_presentation_support(
                            physical_device,
                            queue_family_index,
                            connection.as_ptr().cast(),
                            visual_id.get(),
                        )
                }

                // Android, OHOS, and Metal have no presentation support query.
                #[cfg(target_os = "android")]
                (RawDisplayHandle::Android(_), _) => true,

                #[cfg(target_env = "ohos")]
                (RawDisplayHandle::Ohos(_), _) => true,

                #[cfg(any(target_os = "macos", target_os = "ios"))]
                (RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_), _) => true,

                _ => false,
            }
        }
    }

    /// Create a [`vk::SurfaceKHR`] from raw display and window handles.
    ///
    /// # Safety
    ///
    /// - `instance` must be a valid Vulkan instance.
    /// - The window and display handles must be valid and must outlive the returned surface.
    /// - The returned surface must be destroyed before the instance.
    pub unsafe fn create_surface(
        &self,
        instance: vk::Instance,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            match (display_handle, window_handle) {
                #[cfg(target_os = "windows")]
                (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
                    let fns = self
                        .win32
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::Win32SurfaceCreateInfoKHR::default()
                        .hwnd(window.hwnd.get() as crate::HWND)
                        .hinstance(
                            window
                                .hinstance
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .get() as crate::HINSTANCE,
                        );
                    fns.create_win32_surface(instance, &create_info, allocator)
                }

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                    let fns = self
                        .wayland
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::WaylandSurfaceCreateInfoKHR::default()
                        .display(display.display.as_ptr().cast())
                        .surface(window.surface.as_ptr().cast());
                    fns.create_wayland_surface(instance, &create_info, allocator)
                }

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                    let fns = self
                        .xlib
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::XlibSurfaceCreateInfoKHR::default()
                        .dpy(
                            display
                                .display
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .as_ptr()
                                .cast(),
                        )
                        .window(window.window);
                    fns.create_xlib_surface(instance, &create_info, allocator)
                }

                #[cfg(all(
                    not(target_env = "ohos"),
                    any(
                        target_os = "linux",
                        target_os = "dragonfly",
                        target_os = "freebsd",
                        target_os = "netbsd",
                        target_os = "openbsd",
                    ),
                ))]
                (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                    let fns = self
                        .xcb
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::XcbSurfaceCreateInfoKHR::default()
                        .connection(
                            display
                                .connection
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .as_ptr()
                                .cast(),
                        )
                        .window(window.window.get());
                    fns.create_xcb_surface(instance, &create_info, allocator)
                }

                #[cfg(target_os = "android")]
                (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
                    let fns = self
                        .android
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::AndroidSurfaceCreateInfoKHR::default()
                        .window(window.a_native_window.as_ptr().cast());
                    fns.create_android_surface(instance, &create_info, allocator)
                }

                #[cfg(target_env = "ohos")]
                (RawDisplayHandle::Ohos(_), RawWindowHandle::OhosNdk(window)) => {
                    let fns = self
                        .ohos
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    let create_info = vk::SurfaceCreateInfoOHOS::default()
                        .window(window.native_window.as_ptr().cast());
                    fns.create_surface_ohos(instance, &create_info, allocator)
                }

                #[cfg(target_os = "macos")]
                (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
                    let fns = self
                        .metal
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    use raw_window_metal::{Layer, appkit};

                    let layer = match appkit::metal_layer_from_handle(window) {
                        Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                    };

                    let create_info = vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                    fns.create_metal_surface(instance, &create_info, allocator)
                }

                #[cfg(target_os = "ios")]
                (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
                    let fns = self
                        .metal
                        .as_ref()
                        .ok_or(vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    use raw_window_metal::{Layer, uikit};

                    let layer = match uikit::metal_layer_from_handle(window) {
                        Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                    };

                    let create_info = vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                    fns.create_metal_surface(instance, &create_info, allocator)
                }

                _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }
}

/// Returns all surface-related instance extensions valid on this platform.
///
/// Unlike [`required_extensions()`], this does not require a display handle —
/// it returns the union of all platform-possible surface extensions so that callers can
/// enable them all up front.
///
/// The returned set always includes `VK_KHR_surface`.
pub fn desired_extensions() -> InstanceExtensionSet {
    let mut set = InstanceExtensionSet::empty();
    let _ = set.insert(vk::khr::surface::EXTENSION_NAME);

    #[cfg(target_os = "windows")]
    let _ = set.insert(vk::khr::win32_surface::EXTENSION_NAME);

    #[cfg(all(
        not(target_env = "ohos"),
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ),
    ))]
    {
        let _ = set.insert(vk::khr::wayland_surface::EXTENSION_NAME);
        let _ = set.insert(vk::khr::xlib_surface::EXTENSION_NAME);
        let _ = set.insert(vk::khr::xcb_surface::EXTENSION_NAME);
    }

    #[cfg(target_os = "android")]
    let _ = set.insert(vk::khr::android_surface::EXTENSION_NAME);

    #[cfg(target_env = "ohos")]
    let _ = set.insert(vk::ohos::surface::EXTENSION_NAME);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let _ = set.insert(vk::ext::metal_surface::EXTENSION_NAME);

    set
}

/// Query the required instance extensions for creating a surface from a display handle.
///
/// The returned set always includes `VK_KHR_surface` plus the platform-specific surface
/// extension matching the given display handle.
pub fn required_extensions(
    display_handle: RawDisplayHandle,
) -> crate::Result<InstanceExtensionSet> {
    let mut set = InstanceExtensionSet::empty();
    let _ = set.insert(vk::khr::surface::EXTENSION_NAME);

    let ext = match display_handle {
        #[cfg(target_os = "windows")]
        RawDisplayHandle::Windows(_) => vk::khr::win32_surface::EXTENSION_NAME,

        #[cfg(all(
            not(target_env = "ohos"),
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ),
        ))]
        RawDisplayHandle::Wayland(_) => vk::khr::wayland_surface::EXTENSION_NAME,

        #[cfg(all(
            not(target_env = "ohos"),
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ),
        ))]
        RawDisplayHandle::Xlib(_) => vk::khr::xlib_surface::EXTENSION_NAME,

        #[cfg(all(
            not(target_env = "ohos"),
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ),
        ))]
        RawDisplayHandle::Xcb(_) => vk::khr::xcb_surface::EXTENSION_NAME,

        #[cfg(target_os = "android")]
        RawDisplayHandle::Android(_) => vk::khr::android_surface::EXTENSION_NAME,

        #[cfg(target_env = "ohos")]
        RawDisplayHandle::Ohos(_) => vk::ohos::surface::EXTENSION_NAME,

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            vk::ext::metal_surface::EXTENSION_NAME
        }

        _ => return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    let _ = set.insert(ext);
    Ok(set)
}
