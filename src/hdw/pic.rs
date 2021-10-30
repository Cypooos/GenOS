use crate::io;

pub const PIC1_HZ: u32 = 44100;

// Thanks to OSdev.org lol
pub const CMD: u16 = 0x43;
pub const DATA0: u16 = 0x40; // pic 1 connected to IDT
pub const DATA1: u16 = 0x41; // not used
pub const DATA2: u16 = 0x42; // pic 2 connected to speaker w/ command

pub const MAX_RATE: u32 = 1193180;

// TODO: not use outb and use serial::port (remove unsafe)

pub fn set_pic1(hz: u32) {
    let divisor = MAX_RATE.checked_div(hz).unwrap_or(0);
    unsafe {
        io::outb(CMD, 0x36); // command set DATA0
        io::outb(DATA0, (divisor & 0xFF) as u8); // send data
        io::outb(DATA0, (divisor >> 8) as u8);
    }
    // io::outb(0x61, io::inb(0x61) | 3); // start audio

    // outb(0x61, inb(0x61) & 0xFC); // stop audio
}

pub fn start_audio() {
    unsafe {
        io::outb(0x61, io::inb(0x61) | 3);
    }
}

pub fn set_pic2(hz: u32) {
    let divisor = MAX_RATE.checked_div(hz).unwrap_or(0);
    //let microseconds = ((sample * 60) / 255) as u16; // to 8bit
    unsafe {
        io::outb(CMD, 0xb6); // command set DATA2
        io::outb(DATA2, (divisor & 0xFF) as u8); // send data
        io::outb(DATA2, (divisor >> 8) as u8);
    }
}
