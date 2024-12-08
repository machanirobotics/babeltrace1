use super::event::Event;
use babeltrace1_sys::*;
use std::ptr;

pub struct Iterator {
    inner: ptr::NonNull<bt_ctf_iter>,
    idx: usize,
}

impl Iterator {
    pub(crate) fn from_ptr(ptr: *mut bt_ctf_iter) -> Option<Iterator> {
        Some(Iterator {
            inner: ptr::NonNull::new(ptr)?,
            idx: 0,
        })
    }
}

impl std::iter::Iterator for Iterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.idx != 0 {
                let ret = bt_iter_next(bt_ctf_get_iter(self.inner.as_ptr()));
                if ret < 0 {
                    return None;
                }
            }
            self.idx += 1;

            let event_ptr = bt_ctf_iter_read_event(self.inner.as_ptr());
            Event::from_ptr(event_ptr)
        }
    }
}

impl Drop for Iterator {
    fn drop(&mut self) {
        unsafe {
            bt_ctf_iter_destroy(self.inner.as_ptr());
        }
    }
}
