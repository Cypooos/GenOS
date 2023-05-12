use crate::interface::events::{InterruptEvent,self};
use crate::debug;

pub fn default_interface() -> ! {
    let mut time:usize = 0;
    loop {
        match events::get() {
            None => (),
            Some(InterruptEvent::Time) => {
                time +=1;
                debug!("OK ! Time is: {}",time);
            },
            Some(InterruptEvent::Key(_k)) => {},
            Some(InterruptEvent::Mouse) => {},
        }
    }
}