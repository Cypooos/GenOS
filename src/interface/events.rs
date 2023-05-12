use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::hdw::allocator::HAVE_ALLOC;

lazy_static! {
    static ref EVENTS: Mutex<Vec<InterruptEvent>> = Mutex::new(Vec::new());
}

pub enum InterruptEvent {
    Time,
    Key(u8),
    Mouse,
}

pub fn get() -> Option<InterruptEvent> {
    match EVENTS.try_lock() {
        None => None,
        Some(mut e) => {
            e.pop()
        }
    }

}

impl InterruptEvent {
    pub fn push(self) {
        match HAVE_ALLOC.try_lock() {
            Some(mg) =>
            if *mg {
                match EVENTS.try_lock() {
                    None => (),
                    Some(mut v) =>{
                        v.push(self)
                    }
                }
            },
            None => ()
        }
    }
}