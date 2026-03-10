//! Interop between kazan and [`raw_window_handle`].
//!
//! Provides [`InstanceFn`] to load platform-specific surface creation function pointers and
//! create [`vk::SurfaceKHR`] handles, and [`enumerate_required_extensions()`] to query which
//! instance extensions are needed.

use core::ffi::{CStr, c_char};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::{LoadInstanceFn as _, vk};

/// Combines an [`InstanceFn`] with a [`RawDisplayHandle`], so callers only need to pass
/// a [`RawWindowHandle`] when creating surfaces.
pub struct SurfaceFactory {
    instance_fn: InstanceFn,
    display_handle: RawDisplayHandle,
}

/// Loaded platform-specific surface creation function pointers.
///
/// Constructed via [`InstanceFn::load()`], which loads the appropriate function pointers
/// based on the [`RawDisplayHandle`]. Call [`create_surface()`](InstanceFn::create_surface)
/// to create a [`vk::SurfaceKHR`].
pub enum InstanceFn {
    #[cfg(target_os = "windows")]
    Win32(vk::khr::win32_surface::InstanceFn),

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Wayland(vk::khr::wayland_surface::InstanceFn),

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Xlib(vk::khr::xlib_surface::InstanceFn),

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Xcb(vk::khr::xcb_surface::InstanceFn),

    #[cfg(target_os = "android")]
    Android(vk::khr::android_surface::InstanceFn),

    #[cfg(target_env = "ohos")]
    Ohos(vk::ohos::surface::InstanceFn),

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    Metal(vk::ext::metal_surface::InstanceFn),
}

impl SurfaceFactory {
    /// Load the platform-specific surface extension for the given display and instance.
    ///
    /// # Safety
    ///
    /// `instance` must be a valid Vulkan instance with the required surface extensions enabled.
    /// The display handle must remain valid for the lifetime of this `SurfaceFactory`.
    pub unsafe fn new(
        entry: &crate::Entry,
        instance: vk::Instance,
        display_handle: RawDisplayHandle,
    ) -> crate::Result<Self> {
        let instance_fn = unsafe { InstanceFn::load(entry, instance, display_handle)? };
        Ok(Self {
            instance_fn,
            display_handle,
        })
    }

    /// Load using a raw function pointer loader.
    ///
    /// # Safety
    ///
    /// The `load` function must return valid function pointers for the given instance.
    /// The display handle must remain valid for the lifetime of this `SurfaceFactory`.
    pub unsafe fn new_with(
        load: impl Fn(&CStr) -> Option<vk::PFN_vkVoidFunction>,
        display_handle: RawDisplayHandle,
    ) -> crate::Result<Self> {
        let instance_fn = unsafe { InstanceFn::load_with(load, display_handle)? };
        Ok(Self {
            instance_fn,
            display_handle,
        })
    }

    /// Query whether the given queue family on a physical device supports presentation
    /// to this factory's display system.
    ///
    /// See [`InstanceFn::get_physical_device_presentation_support()`] for details on
    /// platform-specific behavior.
    ///
    /// # Safety
    ///
    /// - `physical_device` must be a valid physical device.
    /// - `window_handle` must be valid.
    pub unsafe fn get_physical_device_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        window_handle: RawWindowHandle,
    ) -> bool {
        unsafe {
            self.instance_fn.get_physical_device_presentation_support(
                physical_device,
                queue_family_index,
                self.display_handle,
                window_handle,
            )
        }
    }

    /// Create a [`vk::SurfaceKHR`] from a raw window handle.
    ///
    /// # Safety
    ///
    /// - `instance` must be a valid Vulkan instance.
    /// - The window handle must be valid and must outlive the returned surface.
    /// - The returned surface must be destroyed before the instance.
    pub unsafe fn create_surface(
        &self,
        instance: vk::Instance,
        window_handle: RawWindowHandle,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> crate::Result<vk::SurfaceKHR> {
        unsafe {
            self.instance_fn
                .create_surface(instance, self.display_handle, window_handle, allocator)
        }
    }
}

impl InstanceFn {
    /// Load the platform-specific surface extension for the given display and instance.
    ///
    /// # Safety
    ///
    /// `instance` must be a valid Vulkan instance with the required surface extensions enabled.
    pub unsafe fn load(
        entry: &crate::Entry,
        instance: vk::Instance,
        display_handle: RawDisplayHandle,
    ) -> crate::Result<Self> {
        unsafe {
            Self::load_with(
                |name| {
                    core::mem::transmute((entry.static_fn.get_instance_proc_addr)(
                        instance,
                        name.as_ptr(),
                    ))
                },
                display_handle,
            )
        }
    }

    /// Load the platform-specific surface extension using a raw function pointer loader.
    ///
    /// # Safety
    ///
    /// The `load` function must return valid function pointers for the given instance.
    pub unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<vk::PFN_vkVoidFunction>,
        display_handle: RawDisplayHandle,
    ) -> crate::Result<Self> {
        unsafe {
            match display_handle {
                #[cfg(target_os = "windows")]
                RawDisplayHandle::Windows(_) => {
                    let fns = vk::khr::win32_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Win32(fns))
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                RawDisplayHandle::Wayland(_) => {
                    let fns = vk::khr::wayland_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Wayland(fns))
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                RawDisplayHandle::Xlib(_) => {
                    let fns = vk::khr::xlib_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Xlib(fns))
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                RawDisplayHandle::Xcb(_) => {
                    let fns = vk::khr::xcb_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Xcb(fns))
                }

                #[cfg(target_os = "android")]
                RawDisplayHandle::Android(_) => {
                    let fns = vk::khr::android_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Android(fns))
                }

                #[cfg(target_env = "ohos")]
                RawDisplayHandle::Ohos(_) => {
                    let fns = vk::ohos::surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Ohos(fns))
                }

                #[cfg(any(target_os = "macos", target_os = "ios"))]
                RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
                    let fns = vk::ext::metal_surface::InstanceFn::load_with(&load)
                        .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                    Ok(Self::Metal(fns))
                }

                _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    /// Query whether the given queue family on a physical device supports presentation
    /// to the display system associated with this `InstanceFn`.
    ///
    /// Returns `true` on platforms that have no dedicated presentation support query
    /// (Android, OHOS, Metal), or when the Xlib/Xcb window handle has no `visual_id`.
    ///
    /// # Safety
    ///
    /// - `physical_device` must be a valid physical device.
    /// - `display_handle` and `window_handle` must be valid and match this `InstanceFn` variant.
    pub unsafe fn get_physical_device_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> bool {
        unsafe {
            match (self, display_handle, window_handle) {
                #[cfg(target_os = "windows")]
                (Self::Win32(fns), RawDisplayHandle::Windows(_), _) => fns
                    .get_physical_device_win32_presentation_support_khr(
                        physical_device,
                        queue_family_index,
                    ),

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (Self::Wayland(fns), RawDisplayHandle::Wayland(display), _) => fns
                    .get_physical_device_wayland_presentation_support_khr(
                        physical_device,
                        queue_family_index,
                        display.display.as_ptr().cast(),
                    ),

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (
                    Self::Xlib(fns),
                    RawDisplayHandle::Xlib(display),
                    RawWindowHandle::Xlib(window),
                ) => {
                    let Some(dpy) = display.display else {
                        return true;
                    };
                    if window.visual_id == 0 {
                        return true;
                    }
                    fns.get_physical_device_xlib_presentation_support_khr(
                        physical_device,
                        queue_family_index,
                        dpy.as_ptr().cast(),
                        window.visual_id as crate::VisualID,
                    )
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (Self::Xcb(fns), RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                    let Some(connection) = display.connection else {
                        return true;
                    };
                    let Some(visual_id) = window.visual_id else {
                        return true;
                    };
                    fns.get_physical_device_xcb_presentation_support_khr(
                        physical_device,
                        queue_family_index,
                        connection.as_ptr().cast(),
                        visual_id.get(),
                    )
                }

                // Android, OHOS, and Metal have no presentation support query.
                _ => true,
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
            match (self, display_handle, window_handle) {
                #[cfg(target_os = "windows")]
                (
                    Self::Win32(fns),
                    RawDisplayHandle::Windows(_),
                    RawWindowHandle::Win32(window),
                ) => {
                    let create_info = vk::Win32SurfaceCreateInfoKHR::default()
                        .hwnd(window.hwnd.get() as crate::HWND)
                        .hinstance(
                            window
                                .hinstance
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .get() as crate::HINSTANCE,
                        );
                    fns.create_win32_surface_khr(instance, &create_info, allocator)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (
                    Self::Wayland(fns),
                    RawDisplayHandle::Wayland(display),
                    RawWindowHandle::Wayland(window),
                ) => {
                    let create_info = vk::WaylandSurfaceCreateInfoKHR::default()
                        .display(display.display.as_ptr().cast())
                        .surface(window.surface.as_ptr().cast());
                    fns.create_wayland_surface_khr(instance, &create_info, allocator)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (
                    Self::Xlib(fns),
                    RawDisplayHandle::Xlib(display),
                    RawWindowHandle::Xlib(window),
                ) => {
                    let create_info = vk::XlibSurfaceCreateInfoKHR::default()
                        .dpy(
                            display
                                .display
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .as_ptr()
                                .cast(),
                        )
                        .window(window.window);
                    fns.create_xlib_surface_khr(instance, &create_info, allocator)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd",
                ))]
                (Self::Xcb(fns), RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                    let create_info = vk::XcbSurfaceCreateInfoKHR::default()
                        .connection(
                            display
                                .connection
                                .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                                .as_ptr()
                                .cast(),
                        )
                        .window(window.window.get());
                    fns.create_xcb_surface_khr(instance, &create_info, allocator)
                }

                #[cfg(target_os = "android")]
                (
                    Self::Android(fns),
                    RawDisplayHandle::Android(_),
                    RawWindowHandle::AndroidNdk(window),
                ) => {
                    let create_info = vk::AndroidSurfaceCreateInfoKHR::default()
                        .window(window.a_native_window.as_ptr().cast());
                    fns.create_android_surface_khr(instance, &create_info, allocator)
                }

                #[cfg(target_env = "ohos")]
                (Self::Ohos(fns), RawDisplayHandle::Ohos(_), RawWindowHandle::OhosNdk(window)) => {
                    let create_info = vk::SurfaceCreateInfoOHOS::default()
                        .window(window.native_window.as_ptr().cast());
                    fns.create_surface_ohos(instance, &create_info, allocator)
                }

                #[cfg(target_os = "macos")]
                (
                    Self::Metal(fns),
                    RawDisplayHandle::AppKit(_),
                    RawWindowHandle::AppKit(window),
                ) => {
                    use raw_window_metal::{Layer, appkit};

                    let layer = match appkit::metal_layer_from_handle(window) {
                        Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                    };

                    let create_info = vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                    fns.create_metal_surface_ext(instance, &create_info, allocator)
                }

                #[cfg(target_os = "ios")]
                (Self::Metal(fns), RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
                    use raw_window_metal::{Layer, uikit};

                    let layer = match uikit::metal_layer_from_handle(window) {
                        Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                    };

                    let create_info = vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                    fns.create_metal_surface_ext(instance, &create_info, allocator)
                }

                _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }
}

/// Query the required instance extensions for creating a surface from a display handle.
///
/// The returned extensions include all extension dependencies (i.e. `VK_KHR_surface` is always
/// included).
pub fn enumerate_required_extensions(
    display_handle: RawDisplayHandle,
) -> crate::Result<&'static [*const c_char]> {
    let extensions = match display_handle {
        #[cfg(target_os = "windows")]
        RawDisplayHandle::Windows(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::khr::win32_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        RawDisplayHandle::Wayland(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::khr::wayland_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        RawDisplayHandle::Xlib(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::khr::xlib_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        RawDisplayHandle::Xcb(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::khr::xcb_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(target_os = "android")]
        RawDisplayHandle::Android(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::khr::android_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(target_env = "ohos")]
        RawDisplayHandle::Ohos(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::ohos::surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            const EXTS: [*const c_char; 2] = [
                vk::khr::surface::EXTENSION_NAME.as_ptr(),
                vk::ext::metal_surface::EXTENSION_NAME.as_ptr(),
            ];
            &EXTS[..]
        }

        _ => return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    Ok(extensions)
}
