# Kazan

[![CI](https://github.com/jwollen/kazan/actions/workflows/ci.yml/badge.svg)](https://github.com/jwollen/kazan/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![MSRV](https://img.shields.io/badge/MSRV-1.89%2B-%23dea584.svg)](https://blog.rust-lang.org/)

Vulkan bindings for Rust, generated from the official [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) `vk.xml` registry.

## Modules

- **`kazan::*`** -- helper types and traits (`Handle`, `ApiVersion`, `ArrayCStr`, `InstanceExtensionSet`, `DeviceExtensionSet`, `RawPtr`, `TaggedStructure`, `Extends`, etc.)
- **`vk`** -- all Vulkan types, constants, and function pointer types, re-exported flat
  - **`vk::vk1_0`**, **`vk::vk1_1`**, ... - items grouped by API version
  - **`vk::khr`**, **`vk::ext`**, **`vk::nv`**, ... - items grouped by vendor
  - **`vk::khr::swapchain`**, **`vk::ext::debug_utils`**, ... - individual extensions
  - **`vk::video::*`** - video extensions
- **`window`** -- platform surface creation via `raw-window-handle` (requires `window` feature)
- **`ffi`** -- raw C FFI function types (requires `ffi` feature)

## Function tables

Each API version and extension defines its own `EntryFn`, `InstanceFn`, and/or `DeviceFn` struct containing the commands it introduces.

- **`EntryFn`** -- global commands, no instance required
  - `vk::vk1_0::EntryFn` -- `create_instance`, `enumerate_instance_extension_properties`, ...
  - `vk::vk1_1::EntryFn` -- `enumerate_instance_version`
- **`InstanceFn`** -- loaded via `vk::vk1_0::EntryFn`
  - `vk::vk1_0::InstanceFn` -- `enumerate_physical_devices`, `create_device`, `get_device_proc_addr`, ...
  - `vk::khr::surface::InstanceFn` -- `get_physical_device_surface_support`, `destroy_surface`, ...
  - `vk::ext::debug_utils::InstanceFn` -- `create_debug_utils_messenger`, ...
- **`DeviceFn`** -- loaded via `vk::vk1_0::InstanceFn`
  - `vk::vk1_0::DeviceFn` -- `create_image`, `cmd_draw`, `queue_submit`, ...
  - `vk::khr::swapchain::DeviceFn` -- `create_swapchain`, `acquire_next_image`, ...

`Entry` bootstraps the loading chain: it holds `StaticFn` (`vkGetInstanceProcAddr`), `vk1_0::EntryFn`, and optionally `vk1_1::EntryFn`.

### Example

```rust
use kazan::{Entry, LoadDeviceFn as _, LoadInstanceFn as _, vk::{self, khr::swapchain, vk1_0}};

// Load the Vulkan library and entry points.
let entry = unsafe { Entry::load()? };

// Create an instance with surface extensions for the current platform.
let display_handle = window.display_handle()?.as_raw();
let required = kazan::window::required_extensions(display_handle)?;
let extension_names: Vec<*const c_char> = required.names().map(|n| n.as_ptr()).collect();
let app_info = vk::ApplicationInfo::default()
    .application_name(c"MyApp")
    .api_version(vk1_0::API_VERSION);
let create_info = vk::InstanceCreateInfo::default()
    .application_info(&app_info)
    .enabled_extension_names(&extension_names);
let instance = entry.vk1_0.create_instance(&create_info, None)?;

// Load instance function tables.
let instance_fn = unsafe { vk1_0::InstanceFn::load(&entry, instance)? };

// Create a surface via the platform-agnostic window module.
let window_fn = kazan::window::InstanceFn::load(&entry, instance);
let window_handle = window.window_handle()?.as_raw();
let surface = unsafe { window_fn.create_surface(instance, display_handle, window_handle, None)? };

// Create a device and load device function tables.
let device = unsafe {
    instance_fn.create_device(physical_device, &device_create_info, None)?
};
let device_fn = unsafe { vk1_0::DeviceFn::load(&instance_fn, device)? };
let swapchain_fn = unsafe { swapchain::DeviceFn::load(&instance_fn, device)? };
```

## Features

| Feature | Description |
|---|---|
| `loaded` | Runtime loading of the Vulkan library via `libloading` |
| `linked` | Link to the system Vulkan library at build time |
| `std` | Standard library support (`read_spv`, `Error` impls) |
| `debug` | `Debug` trait implementations for generated types |
| `window` | Platform surface creation via `raw-window-handle` |
| `ffi` | Expose raw C FFI types in `vk::ffi` |
| `provisional` | Include provisional/beta extensions |

## Running the generator

Parses `vk.xml` and `video.xml` from the vendored [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) submodule and writes generated source to `crates/kazan/src/generated/`.

```bash
cargo run -p generator
```

## Acknowledgements

Kazan is heavily based on [ash](https://github.com/ash-rs/ash).

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
