//! Interop between kazan and [`raw_window_handle`].
//!
//! Provides [`create_surface()`] to create a [`vk::SurfaceKHR`] from a raw window/display handle,
//! and [`enumerate_required_extensions()`] to query which instance extensions are needed.

use core::ffi::{c_char, CStr};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::vk;

/// Create a [`vk::SurfaceKHR`] from raw display and window handles.
///
/// The caller must pass a `load` function that resolves Vulkan instance-level function pointers
/// (typically via `vkGetInstanceProcAddr` for the given instance).
///
/// The instance must have been created with the platform-specific surface extensions enabled,
/// as returned by [`enumerate_required_extensions()`].
///
/// # Safety
///
/// - `instance` must be a valid Vulkan instance.
/// - The window and display handles must be valid and must outlive the returned surface.
/// - The returned surface must be destroyed before the instance.
pub unsafe fn create_surface(
    load: impl Fn(&CStr) -> Option<vk::PFN_vkVoidFunction>,
    instance: vk::Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocator: Option<&vk::AllocationCallbacks<'_>>,
) -> crate::Result<vk::SurfaceKHR> {
    unsafe {
        match (display_handle, window_handle) {
            #[cfg(target_os = "windows")]
            (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
                let surface_fn = vk::khr::win32_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info = vk::Win32SurfaceCreateInfoKHR::default()
                    .hwnd(window.hwnd.get() as crate::HWND)
                    .hinstance(
                        window
                            .hinstance
                            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                            .get() as crate::HINSTANCE,
                    );
                surface_fn.create_win32_surface_khr(instance, &create_info, allocator)
            }

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                let surface_fn = vk::khr::wayland_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info = vk::WaylandSurfaceCreateInfoKHR::default()
                    .display(display.display.as_ptr().cast())
                    .surface(window.surface.as_ptr().cast());
                surface_fn.create_wayland_surface_khr(instance, &create_info, allocator)
            }

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                let surface_fn = vk::khr::xlib_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info = vk::XlibSurfaceCreateInfoKHR::default()
                    .dpy(
                        display
                            .display
                            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                            .as_ptr()
                            .cast(),
                    )
                    .window(window.window);
                surface_fn.create_xlib_surface_khr(instance, &create_info, allocator)
            }

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                let surface_fn = vk::khr::xcb_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info = vk::XcbSurfaceCreateInfoKHR::default()
                    .connection(
                        display
                            .connection
                            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                            .as_ptr()
                            .cast(),
                    )
                    .window(window.window.get());
                surface_fn.create_xcb_surface_khr(instance, &create_info, allocator)
            }

            #[cfg(target_os = "android")]
            (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
                let surface_fn = vk::khr::android_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info = vk::AndroidSurfaceCreateInfoKHR::default()
                    .window(window.a_native_window.as_ptr().cast());
                surface_fn.create_android_surface_khr(instance, &create_info, allocator)
            }

            #[cfg(target_os = "macos")]
            (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
                use raw_window_metal::{appkit, Layer};

                let layer = match appkit::metal_layer_from_handle(window) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                };

                let surface_fn = vk::ext::metal_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info =
                    vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                surface_fn.create_metal_surface_ext(instance, &create_info, allocator)
            }

            #[cfg(target_os = "ios")]
            (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
                use raw_window_metal::{uikit, Layer};

                let layer = match uikit::metal_layer_from_handle(window) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                };

                let surface_fn = vk::ext::metal_surface::InstanceFn::load(&load)
                    .map_err(|_| vk::Result::ERROR_EXTENSION_NOT_PRESENT)?;
                let create_info =
                    vk::MetalSurfaceCreateInfoEXT::default().p_layer(&*layer);
                surface_fn.create_metal_surface_ext(instance, &create_info, allocator)
            }

            _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
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
