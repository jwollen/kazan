mod generated;
pub use generated::*;

use kazan_sys::vk::Result;
use std::mem::MaybeUninit;

pub trait ExtendUninit<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>];

    unsafe fn set_len(&mut self, len: usize);
}

impl<T> ExtendUninit<T> for Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        self.reserve(capacity.saturating_div(self.capacity()));
        self.spare_capacity_mut()
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe {
            self.set_len(self.len() + len);
        }
    }
}

impl<T> ExtendUninit<T> for &mut Vec<T> {
    unsafe fn reserve(&mut self, capacity: usize) -> &mut [MaybeUninit<T>] {
        unsafe { ExtendUninit::reserve(*self, capacity) }
    }

    unsafe fn set_len(&mut self, len: usize) {
        unsafe { ExtendUninit::set_len(*self, len) }
    }
}

pub(crate) fn try_extend_uninit<T, N, E, F>(mut e: E, mut f: F) -> Result
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E: ExtendUninit<T>,
    F: FnMut(&mut N, *mut T) -> Result,
{
    let mut len = N::default();
    let result = f(&mut len, std::ptr::null_mut());
    if result != Result::SUCCESS {
        return result;
    }

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data = unsafe { e.reserve(capacity) };
    len = data.len().try_into().unwrap();
    let result = f(&mut len, data.as_mut_ptr() as *mut T);
    if result != Result::SUCCESS {
        return result;
    }
    unsafe { e.set_len(len.try_into().unwrap()) };
    result
}

pub(crate) fn try_extend_uninit2<T1, T2, N, E1, E2, F>(mut e1: E1, mut e2: E2, mut f: F) -> Result
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E1: ExtendUninit<T1>,
    E2: ExtendUninit<T2>,
    F: FnMut(&mut N, *mut T1, *mut T2) -> Result,
{
    let mut len = N::default();
    let result = f(&mut len, std::ptr::null_mut(), std::ptr::null_mut());
    if result != Result::SUCCESS {
        return result;
    }

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data1 = unsafe { e1.reserve(capacity) };
    let data2 = unsafe { e2.reserve(capacity) };

    len = data1.len().try_into().unwrap();
    assert_eq!(data1.len(), data2.len());

    let result = f(
        &mut len,
        data1.as_mut_ptr() as *mut T1,
        data2.as_mut_ptr() as *mut T2,
    );
    if result != Result::SUCCESS {
        return result;
    }
    unsafe {
        e1.set_len(len.try_into().unwrap());
        e2.set_len(len.try_into().unwrap())
    };
    result
}

pub(crate) fn extend_uninit<T, N, E, F>(mut e: E, mut f: F)
where
    N: Copy + Default + TryInto<usize> + TryFrom<usize>,
    <N as TryInto<usize>>::Error: core::fmt::Debug,
    <N as TryFrom<usize>>::Error: core::fmt::Debug,
    E: ExtendUninit<T>,
    F: FnMut(&mut N, *mut T),
{
    let mut len = N::default();
    f(&mut len, std::ptr::null_mut());

    let capacity = len.try_into().expect("failed to convert `N` to usize");
    let data = unsafe { e.reserve(capacity) };
    len = data.len().try_into().unwrap();
    f(&mut len, data.as_mut_ptr() as *mut T);
    unsafe { e.set_len(len.try_into().unwrap()) };
}
