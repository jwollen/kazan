use crate::vk::{BaseOutStructure, StructureType};

/// Structures implementing this trait are layout-compatible with [`BaseInStructure`] and
/// [`BaseOutStructure`]. Such structures have an `s_type` field indicating its type, which must
/// always match the value of [`TaggedStructure::STRUCTURE_TYPE`].
pub unsafe trait TaggedStructure<'a>: Sized {
    const STRUCTURE_TYPE: StructureType;

    /// Prepends the given extension struct between the root and the first pointer. This method is
    /// only available on structs that can be passed to a function directly. Only valid extension
    /// structs can be pushed into the chain.
    /// If the chain looks like `A -> B -> C`, and you call `A.push(&mut D)`, then the
    /// chain will look like `A -> D -> B -> C`.
    ///
    /// # Panics
    /// If `next` contains a pointer chain of its own, this function will panic.  Call `unsafe`
    /// [`Self::extend()`] to insert this chain instead.
    fn push<'b: 'a, T: Extends<Self> + TaggedStructure<'b>>(mut self, next: &'a mut T) -> Self {
        // SAFETY: All implementers of `TaggedStructure` are required to have the `BaseOutStructure` layout
        let slf_base = unsafe { &mut *<*mut _>::cast::<BaseOutStructure<'_>>(&mut self) };
        // SAFETY: All implementers of `T: TaggedStructure` are required to have the `BaseOutStructure` layout
        let next_base = unsafe { &mut *<*mut T>::cast::<BaseOutStructure<'_>>(next) };
        // `next` here can contain a pointer chain.  This function refuses to insert the struct,
        // in favour of calling unsafe extend().
        assert!(
            next_base.p_next.is_null(),
            "push() expects a struct without an existing p_next pointer chain (equal to NULL)"
        );
        next_base.p_next = slf_base.p_next;
        slf_base.p_next = next_base;
        self
    }

    /// Prepends the given extension struct between the root and the first pointer. This method is
    /// only available on structs that can be passed to a function directly. Only valid extension
    /// structs can be pushed into the chain.
    /// If the chain looks like `A -> B -> C` and `D -> E`, and you call `A.extend(&mut D)`,
    /// then the chain will look like `A -> D -> E -> B -> C`.
    ///
    /// # Safety
    /// This function will walk the [`BaseOutStructure::p_next`] chain of `next`, requiring
    /// all non-`NULL` pointers to point to a valid Vulkan structure starting with the
    /// [`BaseOutStructure`] layout.
    ///
    /// The last struct in this chain (i.e. the one where `p_next` is `NULL`) must be writable
    /// memory, as its `p_next` field will be updated with the value of `self.p_next`.
    unsafe fn extend<'b: 'a, T: Extends<Self> + TaggedStructure<'b>>(
        mut self,
        next: &'a mut T,
    ) -> Self {
        // `next` here can contain a pointer chain. This means that we must correctly attach he head
        // to the root and the tail to the rest of the chain For example:
        //
        // next = A -> B
        // Before: `Root -> C -> D -> E`
        // After: `Root -> A -> B -> C -> D -> E`
        //                 ^^^^^^
        //                 next chain
        let slf_base = unsafe { &mut *<*mut _>::cast::<BaseOutStructure<'_>>(&mut self) };
        let next_base = <*mut T>::cast::<BaseOutStructure<'_>>(next);
        let last_next = unsafe { ptr_chain_iter(next).last().unwrap() };
        unsafe { (*last_next).p_next = slf_base.p_next };
        slf_base.p_next = next_base;
        self
    }
}

/// Implemented for every structure that extends base structure `B`. Concretely that means struct
/// `B` is listed in its array of [`structextends` in the Vulkan registry][1].
///
/// Similar to [`TaggedStructure`], all `unsafe` implementers of this trait must guarantee that
/// their structure is layout-compatible [`BaseInStructure`] and [`BaseOutStructure`].
///
/// [1]: https://registry.khronos.org/vulkan/specs/latest/styleguide.html#extensions-interactions
pub unsafe trait Extends<B> {}

/// Iterates through the pointer chain. Includes the item that is passed into the function. Stops at
/// the last [`BaseOutStructure`] that has a null [`BaseOutStructure::p_next`] field.
pub(crate) unsafe fn ptr_chain_iter<'a, T: TaggedStructure<'a>>(
    ptr: &mut T,
) -> impl Iterator<Item = *mut BaseOutStructure<'_>> {
    let ptr = <*mut T>::cast::<BaseOutStructure<'_>>(ptr);
    (0..).scan(ptr, |p_ptr, _| {
        if p_ptr.is_null() {
            return None;
        }
        let n_ptr = unsafe { (**p_ptr).p_next };
        let old = *p_ptr;
        *p_ptr = n_ptr;
        Some(old)
    })
}

/// Given a mutable raw pointer to a type with an `s_type` member such as [`vk::BaseOutStructure`],
/// match on a set of Vulkan structures. The struct will be rebound to the given variable of the
/// type of the given Vulkan structure.
///
/// Note that all match bodies have to be enclosed by curly braces due to macro parsing limitations.
/// It is unfortunately not possible to write `x @ kazan::vk::SomeStruct => one_line_expression(),`.
///
/// ```
/// let mut info = kazan::vk::DeviceCreateInfo::default();
/// let info: *mut kazan::vk::BaseOutStructure<'_> = <*mut _>::cast(&mut info);
/// unsafe {
///     kazan::match_out_struct!(match info {
///         info @ kazan::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///         }
///         info @ kazan::vk::DeviceCreateInfo => {
///             dbg!(&info);
///         }
///     })
/// }
/// ```
///
/// In addition this macro propagates implicit return values just like normal `match` blocks, as
/// long as a default value or expression is provided in the "any" match arm
/// (`_ => { some_value() }`). For the time being said arm must be wrapped in curly braces; an
/// expression like `_ => None` is not yet supported.
///
/// ```
/// # let mut info = kazan::vk::DeviceCreateInfo::default();
/// # let info: *mut kazan::vk::BaseOutStructure<'_> = <*mut _>::cast(&mut info);
/// let device_create_flags: Option<kazan::vk::DeviceCreateFlags> = unsafe {
///     kazan::match_out_struct!(match info {
///         info @ kazan::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///             Some(kazan::vk::DeviceCreateFlags::empty())
///         }
///         info @ kazan::vk::DeviceCreateInfo => {
///             dbg!(&info);
///             Some(info.flags)
///         }
///         _ => {
///             None
///         }
///     })
/// };
/// ```
#[macro_export]
macro_rules! match_out_struct {
    (match $p:ident { $($bind:ident @ $ty:path => $body:block $(,)?)+ $(_ => $any:block $(,)?)? }) => {
        match core::ptr::addr_of!((*$p).s_type).read() {
            $(<$ty as $crate::TaggedStructure>::STRUCTURE_TYPE => {
                let $bind = $p
                    .cast::<$ty>()
                    .as_mut()
                    .unwrap();
                $body
            }),+
            _ => { $($any)? }
        }
    };
}

/// Given an immutable raw pointer to a type with an `s_type` member such as [`vk::BaseInStructure`],
/// match on a set of Vulkan structures. The struct will be rebound to the given variable of the
/// type of the given Vulkan structure.
///
/// Note that all match bodies have to be enclosed by curly braces due to macro parsing limitations.
/// It is unfortunately not possible to write `x @ kazan::vk::SomeStruct => one_line_expression(),`.
///
/// ```
/// let info = kazan::vk::DeviceCreateInfo::default();
/// let info: *const kazan::vk::BaseInStructure<'_> = <*const _>::cast(&info);
/// unsafe {
///     kazan::match_in_struct!(match info {
///         info @ kazan::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///         }
///         info @ kazan::vk::DeviceCreateInfo => {
///             dbg!(&info);
///         }
///     })
/// }
/// ```
///
/// See the [`match_out_struct!`] documentation for an example with implicit return values.
#[macro_export]
macro_rules! match_in_struct {
    (match $p:ident { $($bind:ident @ $ty:path => $body:block $(,)?)+ $(_ => $any:block $(,)?)? }) => {
        match core::ptr::addr_of!((*$p).s_type).read() {
            $(<$ty as $crate::TaggedStructure>::STRUCTURE_TYPE => {
                let $bind = $p
                    .cast::<$ty>()
                    .as_ref()
                    .unwrap();
                $body
            }),+
            _ => { $($any)? }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::TaggedStructure as _;
    use crate::vk;

    #[test]
    fn test_ptr_chains() {
        let mut variable_pointers = vk::PhysicalDeviceVariablePointerFeatures::default();
        let mut corner = vk::PhysicalDeviceCornerSampledImageFeaturesNV::default();
        let chain = vec![
            <*mut _>::cast(&mut variable_pointers),
            <*mut _>::cast(&mut corner),
        ];
        let mut device_create_info = vk::DeviceCreateInfo::default()
            .push(&mut corner)
            .push(&mut variable_pointers);
        let chain2: Vec<*mut vk::BaseOutStructure<'_>> = unsafe {
            super::ptr_chain_iter(&mut device_create_info)
                .skip(1)
                .collect()
        };
        assert_eq!(chain, chain2);
    }

    #[test]
    #[should_panic]
    fn disallow_nested_ptr_chains() {
        let mut generated_commands =
            vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT::default();
        let mut private_data = vk::PhysicalDevicePrivateDataFeatures {
            p_next: <*mut _>::cast(&mut generated_commands),
            ..Default::default()
        };
        let _device_create_info = vk::DeviceCreateInfo::default().push(&mut private_data);
    }

    #[test]
    fn test_nested_ptr_chains() {
        let mut generated_commands =
            vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT::default();
        let mut private_data = vk::PhysicalDevicePrivateDataFeatures {
            p_next: <*mut _>::cast(&mut generated_commands),
            ..Default::default()
        };
        let mut variable_pointers = vk::PhysicalDeviceVariablePointerFeatures::default();
        let mut corner = vk::PhysicalDeviceCornerSampledImageFeaturesNV::default();
        let chain = vec![
            <*mut _>::cast(&mut private_data),
            <*mut _>::cast(&mut generated_commands),
            <*mut _>::cast(&mut variable_pointers),
            <*mut _>::cast(&mut corner),
        ];
        let mut device_create_info = vk::DeviceCreateInfo::default()
            .push(&mut corner)
            .push(&mut variable_pointers);
        // Insert private_data->generated_commands into the chain, such that generate_commands->variable_pointers->corner:
        device_create_info = unsafe { device_create_info.extend(&mut private_data) };
        let chain2: Vec<*mut vk::BaseOutStructure<'_>> = unsafe {
            super::ptr_chain_iter(&mut device_create_info)
                .skip(1)
                .collect()
        };
        assert_eq!(chain, chain2);
    }

    #[test]
    fn test_use_struct_after_pointer_chain() {
        // The negative case of this test, where `pdev_props` stays alive by being
        // used at the end of this function while `api` is invalidly accessed first
        // (resulting in an immutable borrow while mutably borrowed error), exists in
        // `tests/fail/long_lived_root_struct_borrow.rs`.  This test demonstrates the expected usage
        // pattern of dropping `pdev_props` so that `api` no longer becomes mutably borrowed and the
        // properties read from Vulkan can now be accessed by the caller.

        let mut layers = vec![];
        let mut api =
            vk::PhysicalDeviceLayeredApiPropertiesListKHR::default().layered_apis(&mut layers);
        let _pdev_props = vk::PhysicalDeviceProperties2::default().push(&mut api);

        // Access to either variable is allowed because `pdev_props` is no longer used
        dbg!(&api);
        dbg!(&layers);
    }

    #[test]
    fn test_debug_flags() {
        assert_eq!(
            format!(
                "{:?}",
                vk::AccessFlagBits::INDIRECT_COMMAND_READ
                    | vk::AccessFlagBits::VERTEX_ATTRIBUTE_READ
            ),
            "INDIRECT_COMMAND_READ | VERTEX_ATTRIBUTE_READ"
        );
    }

    #[test]
    fn test_debug_enum() {
        assert_eq!(format!("{:?}", vk::ChromaLocation::MIDPOINT), "MIDPOINT");
    }
}
