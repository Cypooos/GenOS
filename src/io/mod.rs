pub mod logger;
#[macro_use]
pub mod qemu;
#[macro_use]
pub mod vga_writer;

#[inline]
pub unsafe fn outb(port: u16, data: u8) {
    asm!("out dx, al",in("dx") port,in("al") data);
}

#[inline]
pub unsafe fn outw(port: u16, data: u16) {
    asm!("out dx, ax",in("dx") port,in("ax") data);
}

#[inline]
pub unsafe fn outl(port: u16, data: u32) {
    asm!("out dx, eax",in("dx") port,in("eax") data);
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let out: u8;
    asm!("in al, dx",in("dx") port,out("al") out);
    out
}

#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let out: u16;
    asm!("in ax, dx",in("dx") port,out("ax") out);
    out
}

#[inline]
pub unsafe fn inl(port: u16) -> u32 {
    let out: u32;
    unsafe {
        asm!("in eax, dx",in("dx") port,out("eax") out);
    };
    out
}
