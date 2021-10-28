use crate::serial;

//port addresses
pub const CMD: u16 = 0x43;
pub const DATA_0: u16 = 0x40;
pub const DATA_1: u16 = 0x41;
pub const DATA_2: u16 = 0x42;

//max phase rate of pit
pub const MAX_RATE: u32 = 1193180;

pub fn set_phase(hz: u32) {
    let divisor = MAX_RATE / hz;

    serial::outb(CMD, 0xb6); // command
    serial::outb(DATA_0, (divisor & 0xFF) as u8); // send data
    serial::outb(DATA_0, (divisor >> 8) as u8);
    // serial::outb(0x61, serial::inb(0x61) | 3); // start audio

    // outb(0x61, inb(0x61) & 0xFC); // stop audio
}

pub fn set_phase_pic2(sample: u32) {
    let divisor = MAX_RATE / sample;
    //let microseconds = ((sample * 60) / 255) as u16; // to 8bit
    serial::outb(CMD, 0xb6); // command
    serial::outb(DATA_2, (divisor & 0xFF) as u8); // send data
    serial::outb(DATA_2, (divisor >> 8) as u8);

    let tmp = serial::inb(0x61);
    if tmp != (tmp | 3) {
        // start audio
        serial::outb(0x61, tmp | 3);
    }
}
