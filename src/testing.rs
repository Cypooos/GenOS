use core::panic::PanicInfo;

use crate::serial;

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    qemu_println!("[failed]");
    println!("[$04failed$!]");
    qemu_println!("Error: {}", info);
    //exit_qemu(QemuExitCode::Failed);
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
        qemu_print!("{}...\t", core::any::type_name::<T>());
        self();
        qemu_println!("[ok]");
        println!("[$0aok$!]");
    }
}



#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    qemu_println!("---------- TESTING ----------");
    println!("---------- TESTING ----------");
    qemu_println!("Running {} tests", tests.len());
    println!("Running {} tests", tests.len());
    for test in tests {
        test.test(); // new
    }
    serial::exit_qemu(serial::QemuExitCode::Success);
}

#[test_case]
fn colored_print_test() {
    print!("<");
    print!("$0agreen $$ $ok $!reset $o");
    print!(">");
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 1);
}
