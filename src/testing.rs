use core::panic::PanicInfo;

use crate::serial;

// our panic handler in test mode
pub fn panic_handler(info: &PanicInfo) -> ! {
    qemu_println!("[failed]");
    println!("[$04failed$!]");
    qemu_println!("Error: {}", info);
    println!("Error: {}", info);

    serial::exit_qemu(serial::QemuExitCode::Failed);
    loop {} // not executed but whatever
}

pub trait Testable {
    fn test(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn test(&self) {
        print!("{}...    ",core::any::type_name::<T>());
        qemu_print!("{}...\t\t", core::any::type_name::<T>());
        self();
        qemu_println!("[ok]");
        println!("[$0aok$!]");
    }
}



pub fn test_runner(tests: &[&dyn Testable]) {
    qemu_println!("\n\n --- Running {} tests --- ", tests.len());
    println!("\n\n --- Running {} tests --- ", tests.len());
    for test in tests {
        test.test(); // new
    }
    println!(" --- All was a succes ! --- \n");
    qemu_println!(" --- All was a succes ! --- \n");
    serial::exit_qemu(serial::QemuExitCode::Success);
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
