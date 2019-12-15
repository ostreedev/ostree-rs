// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2017_6", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2017_6", feature = "dox"))]
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use ostree_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AsyncProgress(Object<ostree_sys::OstreeAsyncProgress, ostree_sys::OstreeAsyncProgressClass, AsyncProgressClass>);

    match fn {
        get_type => || ostree_sys::ostree_async_progress_get_type(),
    }
}

impl AsyncProgress {
    pub fn new() -> AsyncProgress {
        unsafe {
            from_glib_full(ostree_sys::ostree_async_progress_new())
        }
    }

    //pub fn new_and_connect(changed: /*Unimplemented*/Option<Fundamental: Pointer>, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> AsyncProgress {
    //    unsafe { TODO: call ostree_sys:ostree_async_progress_new_and_connect() }
    //}
}

impl Default for AsyncProgress {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ASYNC_PROGRESS: Option<&AsyncProgress> = None;

pub trait AsyncProgressExt: 'static {
    #[cfg(any(feature = "v2019_6", feature = "dox"))]
    fn copy_state<P: IsA<AsyncProgress>>(&self, dest: &P);

    fn finish(&self);

    //#[cfg(any(feature = "v2017_6", feature = "dox"))]
    //fn get(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn get_status(&self) -> Option<GString>;

    fn get_uint(&self, key: &str) -> u32;

    fn get_uint64(&self, key: &str) -> u64;

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn get_variant(&self, key: &str) -> Option<glib::Variant>;

    //#[cfg(any(feature = "v2017_6", feature = "dox"))]
    //fn set(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn set_status(&self, status: Option<&str>);

    fn set_uint(&self, key: &str, value: u32);

    fn set_uint64(&self, key: &str, value: u64);

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn set_variant(&self, key: &str, value: &glib::Variant);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AsyncProgress>> AsyncProgressExt for O {
    #[cfg(any(feature = "v2019_6", feature = "dox"))]
    fn copy_state<P: IsA<AsyncProgress>>(&self, dest: &P) {
        unsafe {
            ostree_sys::ostree_async_progress_copy_state(self.as_ref().to_glib_none().0, dest.as_ref().to_glib_none().0);
        }
    }

    fn finish(&self) {
        unsafe {
            ostree_sys::ostree_async_progress_finish(self.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2017_6", feature = "dox"))]
    //fn get(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ostree_sys:ostree_async_progress_get() }
    //}

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn get_status(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ostree_sys::ostree_async_progress_get_status(self.as_ref().to_glib_none().0))
        }
    }

    fn get_uint(&self, key: &str) -> u32 {
        unsafe {
            ostree_sys::ostree_async_progress_get_uint(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_uint64(&self, key: &str) -> u64 {
        unsafe {
            ostree_sys::ostree_async_progress_get_uint64(self.as_ref().to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn get_variant(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ostree_sys::ostree_async_progress_get_variant(self.as_ref().to_glib_none().0, key.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2017_6", feature = "dox"))]
    //fn set(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ostree_sys:ostree_async_progress_set() }
    //}

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn set_status(&self, status: Option<&str>) {
        unsafe {
            ostree_sys::ostree_async_progress_set_status(self.as_ref().to_glib_none().0, status.to_glib_none().0);
        }
    }

    fn set_uint(&self, key: &str, value: u32) {
        unsafe {
            ostree_sys::ostree_async_progress_set_uint(self.as_ref().to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    fn set_uint64(&self, key: &str, value: u64) {
        unsafe {
            ostree_sys::ostree_async_progress_set_uint64(self.as_ref().to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    #[cfg(any(feature = "v2017_6", feature = "dox"))]
    fn set_variant(&self, key: &str, value: &glib::Variant) {
        unsafe {
            ostree_sys::ostree_async_progress_set_variant(self.as_ref().to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(this: *mut ostree_sys::OstreeAsyncProgress, f: glib_sys::gpointer)
            where P: IsA<AsyncProgress>
        {
            let f: &F = &*(f as *const F);
            f(&AsyncProgress::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AsyncProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AsyncProgress")
    }
}
