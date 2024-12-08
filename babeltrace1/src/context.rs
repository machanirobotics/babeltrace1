use crate::{ctf, Error};
use babeltrace1_sys::*;
use std::{
    ffi::{CStr, CString},
    fmt::Display,
    path::Path,
    ptr,
};

pub enum Format {
    Ctf,
}

impl Format {
    fn as_static_c_str(&self) -> &'static CStr {
        match self {
            Format::Ctf => c"ctf",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TraceHandleId(i32);

impl Display for TraceHandleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Debug)]
pub struct Context {
    inner: ptr::NonNull<bt_context>,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let inner =
            ptr::NonNull::new(unsafe { bt_context_create() }).ok_or(Error::ContextCreation)?;
        Ok(Self { inner })
    }

    pub fn add_trace(
        &mut self,
        path: impl AsRef<Path>,
        format: Format,
    ) -> Result<TraceHandleId, Error> {
        let path = path.as_ref().to_str().ok_or(Error::InvalidTracePath)?;

        let id = unsafe {
            let trace_path = CString::new(path).map_err(|_| Error::InvalidTracePath)?;
            bt_context_add_trace(
                self.inner.as_ptr(),
                trace_path.as_ptr(),
                format.as_static_c_str().as_ptr(),
                None,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if id < 0 {
            return Err(Error::TraceAdd);
        }

        Ok(TraceHandleId(id))
    }

    pub fn remove_trace(&mut self, handle: TraceHandleId) -> Result<(), Error> {
        let ret = unsafe { bt_context_remove_trace(self.inner.as_ptr(), handle.0) };
        if ret < 0 {
            Err(Error::TraceRemove(handle))
        } else {
            Ok(())
        }
    }

    pub fn ctf_iter(&self) -> Option<ctf::Iterator> {
        let iter_ptr = unsafe { bt_ctf_iter_create(self.inner.as_ptr(), ptr::null(), ptr::null()) };
        ctf::Iterator::from_ptr(iter_ptr)
    }
}

unsafe impl Send for Context {}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { bt_context_put(self.inner.as_ptr()) }
    }
}
