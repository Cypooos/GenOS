use crate::interface::events::{InterruptEvent,self};
use crate::debug;

pub fn default_interface() -> ! {
    let mut time:usize = 0;
    loop {
        match events::get() {
            None => (),
            Some(InterruptEvent::Time) => {
                time +=1;
                if time%60 == 0 {
                    debug!("OK ! Time is: {}",time/60);
                }
            },
            Some(InterruptEvent::Key(_k)) => {},
            Some(InterruptEvent::Mouse) => {},
        }
    }
}