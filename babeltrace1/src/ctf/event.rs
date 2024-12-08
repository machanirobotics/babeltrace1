use babeltrace1_sys::*;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    ptr,
};

use crate::Error;

pub struct Event {
    inner: ptr::NonNull<bt_ctf_event>,
}

impl Event {
    pub(crate) fn from_ptr(ptr: *mut bt_ctf_event) -> Option<Event> {
        Some(Event {
            inner: ptr::NonNull::new(ptr)?,
        })
    }

    pub fn name(&self) -> &str {
        unsafe {
            let name_ptr = bt_ctf_event_name(self.inner.as_ptr());
            CStr::from_ptr(name_ptr).to_str().unwrap()
        }
    }

    pub fn timestamp(&self) -> Result<u64, Error> {
        unsafe {
            let value = bt_ctf_get_timestamp(self.inner.as_ptr());
            if value == u64::MAX {
                return Err(Error::InvalidTimestamp);
            }

            Ok(value)
        }
    }

    pub fn get_top_level_scope(&self, scope: Scope) -> Result<Definition, Error> {
        unsafe {
            let scope_ptr = bt_ctf_get_top_level_scope(self.inner.as_ptr(), scope.into());
            Definition::from_ptr(scope_ptr).ok_or(Error::GetEventScope)
        }
    }

    pub fn get_field<'a>(
        &'a self,
        scope: &'a Definition<'_>,
        field_name: impl AsRef<str>,
    ) -> Option<Definition<'_>> {
        unsafe {
            let field_name = CString::new(field_name.as_ref()).unwrap();
            let field_ptr =
                bt_ctf_get_field(self.inner.as_ptr(), scope.as_ptr(), field_name.as_ptr());
            Definition::from_ptr(field_ptr)
        }
    }

    pub fn get_u64<'a>(
        &'a self,
        scope: Option<&'a Definition<'a>>,
        field_name: impl AsRef<str>,
    ) -> Result<u64, Error> {
        match scope {
            Some(s) => self
                .get_field(s, field_name.as_ref())
                .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                .get_u64(),
            None => {
                let scope = self.get_top_level_scope(Scope::EventFields)?;
                self.get_field(&scope, field_name.as_ref())
                    .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                    .get_u64()
            }
        }
    }

    pub fn get_i64<'a>(
        &'a self,
        scope: Option<&'a Definition<'a>>,
        field_name: impl AsRef<str>,
    ) -> Result<i64, Error> {
        match scope {
            Some(s) => self
                .get_field(s, field_name.as_ref())
                .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                .get_i64(),
            None => {
                let scope = self.get_top_level_scope(Scope::EventFields)?;
                self.get_field(&scope, field_name.as_ref())
                    .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                    .get_i64()
            }
        }
    }

    pub fn get_f64<'a>(
        &'a self,
        scope: Option<&'a Definition<'a>>,
        field_name: impl AsRef<str>,
    ) -> Result<f64, Error> {
        match scope {
            Some(s) => self
                .get_field(s, field_name.as_ref())
                .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                .get_f64(),
            None => {
                let scope = self.get_top_level_scope(Scope::EventFields)?;
                self.get_field(&scope, field_name.as_ref())
                    .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                    .get_f64()
            }
        }
    }

    pub fn get_string<'a>(
        &'a self,
        scope: Option<&'a Definition<'a>>,
        field_name: impl AsRef<str>,
    ) -> Result<String, Error> {
        match scope {
            Some(s) => self
                .get_field(s, field_name.as_ref())
                .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                .get_str()
                .map(str::to_owned),
            None => {
                let scope = self.get_top_level_scope(Scope::EventFields)?;
                self.get_field(&scope, field_name.as_ref())
                    .ok_or(Error::UnknownField(field_name.as_ref().to_owned()))?
                    .get_str()
                    .map(str::to_owned)
            }
        }
    }
}

pub struct Definition<'a> {
    inner: *const bt_definition,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Definition<'a> {
    fn from_ptr(ptr: *const bt_definition) -> Option<Definition<'a>> {
        if ptr.is_null() {
            return None;
        }

        Some(Self {
            inner: ptr,
            phantom: PhantomData,
        })
    }

    pub fn get_str(&self) -> Result<&str, Error> {
        unsafe {
            let value_str_ptr = bt_ctf_get_string(self.inner);
            if bt_ctf_field_get_error() < 0 {
                return Err(Error::InvalidValueForField);
            }

            Ok(CStr::from_ptr(value_str_ptr).to_str().unwrap())
        }
    }

    pub fn get_f64(&self) -> Result<f64, Error> {
        unsafe {
            let value = bt_ctf_get_float(self.inner);
            if bt_ctf_field_get_error() < 0 {
                return Err(Error::InvalidValueForField);
            }

            Ok(value)
        }
    }

    pub fn get_i64(&self) -> Result<i64, Error> {
        unsafe {
            let value = bt_ctf_get_int64(self.inner);
            if bt_ctf_field_get_error() < 0 {
                return Err(Error::InvalidValueForField);
            }

            Ok(value)
        }
    }

    pub fn get_u64(&self) -> Result<u64, Error> {
        unsafe {
            let value = bt_ctf_get_uint64(self.inner);
            if bt_ctf_field_get_error() < 0 {
                return Err(Error::InvalidValueForField);
            }

            Ok(value)
        }
    }

    fn as_ptr(&self) -> *const bt_definition {
        self.inner
    }
}

pub enum Scope {
    TracePacketHeader,
    StreamPacketContext,
    StreamEventHeader,
    StreamEventContext,
    EventContext,
    EventFields,
}

impl From<Scope> for bt_ctf_scope {
    fn from(value: Scope) -> Self {
        match value {
            Scope::TracePacketHeader => bt_ctf_scope_BT_TRACE_PACKET_HEADER,
            Scope::StreamPacketContext => bt_ctf_scope_BT_STREAM_PACKET_CONTEXT,
            Scope::StreamEventHeader => bt_ctf_scope_BT_STREAM_EVENT_HEADER,
            Scope::StreamEventContext => bt_ctf_scope_BT_STREAM_EVENT_CONTEXT,
            Scope::EventContext => bt_ctf_scope_BT_EVENT_CONTEXT,
            Scope::EventFields => bt_ctf_scope_BT_EVENT_FIELDS,
        }
    }
}
