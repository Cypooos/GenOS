use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// Role of interrupt : re-route user interactiçon to desktop, + manage inteerupts errors

use crate::hdw::pic;
use crate::{debug, error};
use crate::{hdw::gdt, hlt_loop};
use lazy_static::lazy_static;

use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Breakpoint,
    DoubleFault,
    PageFault,
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
    Mouse = PIC_1_OFFSET + 6,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }

    pub fn send_bye_signal(i: InterruptIndex) {
        unsafe {
            PICS.lock().notify_end_of_interrupt(i.as_u8());
        }
    }

    extern "x86-interrupt" fn page_fault(
        stack_frame: InterruptStackFrame,
        error_code: PageFaultErrorCode,
    ) {
        use x86_64::registers::control::Cr2;

        error!("PAGE FAULT");
        error!("Accessed Address: $0C{:?}", Cr2::read());
        error!("Error Code: $0C{:?}", error_code);
        error!(" $0C{:#?}", stack_frame);
        hlt_loop();
    }

    extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame) {
        /*
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            unsafe { DESKTOP.force_unlock() };
            DESKTOP.lock().int_time();
        });
        */
        InterruptIndex::send_bye_signal(InterruptIndex::Timer);
    }

    extern "x86-interrupt" fn keyboard(_stack_frame: InterruptStackFrame) {
        /*
        use x86_64::instructions::interrupts;
        use x86_64::instructions::port::Port;

        interrupts::without_interrupts(|| {
            let mut port = Port::new(0x60);
            unsafe { DESKTOP.force_unlock() };

            let mut desk = DESKTOP.lock();

            let scancode: u8 = unsafe { port.read() };

            desk.int_key(scancode);
        });
        */
        InterruptIndex::send_bye_signal(InterruptIndex::Keyboard);
        return;
    }

    extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
        error!("BREAKPOINT\n{:#?}", stack_frame);
    }

    extern "x86-interrupt" fn double_fault(
        stack_frame: InterruptStackFrame,
        _error_code: u64,
    ) -> ! {
        error!("DOUBLE-FAULT:\n{:#?}", stack_frame);
        panic!("$0CCan't continue on double fault.");
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // exeptions

        idt.breakpoint.set_handler_fn(InterruptIndex::breakpoint);
        unsafe {
            idt.double_fault
                .set_handler_fn(InterruptIndex::double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(InterruptIndex::page_fault);

        // interupts
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(InterruptIndex::timer);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(InterruptIndex::keyboard);
        idt
    };
}

pub fn init_idt() {
    debug!("Initialisation of the IDT");
    IDT.load();

    debug!("Setting up PIC1={}", pic::PIC1_HZ);
    pic::set_pic1(pic::PIC1_HZ);

    #[cfg(feature = "audio")]
    {
        pic::start_audio();
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
