use core::mem::MaybeUninit;

pub fn zeroed_init<T>() -> T {
    let inner = MaybeUninit::<T>::zeroed(); // Creates zero-initialized uninitialized memory
    unsafe { inner.assume_init() }
}
