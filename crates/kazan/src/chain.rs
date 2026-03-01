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
