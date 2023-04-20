use std::ptr;

use obs_wrapper::{
    prelude::*,
    source::*,
    obs_sys,
    obs_string, wrapper::PtrWrapper,
};


struct BrowserSource {
    inner: *mut obs_sys::obs_source_t,
}


impl BrowserSource {
    fn new(name: impl Into<ObsString>) -> Self {
        let inner;
        unsafe {
            inner = obs_sys::obs_source_create(
                Self::get_id().as_ptr(),
                name.into().as_ptr(),
                ptr::null_mut(),
                ptr::null_mut()
            );
        }
        Self { inner }
    }

    fn new_private(name: impl Into<ObsString>) -> Self {
        let inner;
        unsafe {
            inner = obs_sys::obs_source_create_private(
                Self::get_id().as_ptr(),
                name.into().as_ptr(),
                ptr::null_mut(),
            );
        }
        Self { inner }
    }
}

impl Sourceable for BrowserSource {
    fn get_id() -> ObsString {
        obs_string!("browser_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }

    fn create(create: &mut CreatableSourceContext<Self>, source: SourceContext) -> Self {
        let inner;
        unsafe {
            inner = obs_sys::obs_source_create(
                Self::get_id().as_ptr(),
                ptr::null(),
                create.settings.as_ptr_mut(),
                ptr::null_mut()
            );
        }

        Self { inner }
    }
}
