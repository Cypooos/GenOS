use core::panic::PanicInfo;

use crate::{debug, done, error, info, io::qemu};

// our panic handler in test mode
pub fn panic_handler(info: &PanicInfo) -> ! {
    error!("PANIC");
    error!("{}", info);

    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    super::hlt_loop() // not executed but whatever
}

pub trait Testable {
    fn test(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn test(&self) {
        debug!("{}...\t\t", core::any::type_name::<T>());
        self();
        done!("test done");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    info!("\n\n --- Running {} tests --- ", tests.len());
    for test in tests {
        test.test(); // new
    }
    done!(" --- All was a succes ! --- \n");
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}
//
//#[test_case]
//fn colored_print_test() {
//    print!("<");
//    print!("$0agreen $$ $ok $!reset $o");
//    print!(">");
//}
//
//#[test_case]
//fn trivial_assertion() {
//    assert_eq!(1, 1);
//}
//
//
//#[test_case]
//fn trivial_assertion() {
//    assert_eq!(0, 1);
//}
