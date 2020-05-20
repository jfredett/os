use super::*;

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

/*

test_group {
    test "Foo does a Bar when Baz" {
        assert!(foo.does(bar));
    }

    test "Baz is equal to 4 on it's third invocation" {
        baz(); baz();
        assert_eq!(baz(), 4);
    }
}

-->

mod test {
    use crate::*;
    use super::*;

    #[test_case]
    fn foo_does_a_bar_when_baz() {
        serial_print!("Foo does a Bar when Baz: ");
        assert!(foo.does(bar));
        serial_println!("[ok]");
    }

    #[test_case]
    fn baz_is_equal_to_4_on_it_s_third_invocation() {
        serial_print!("Baz is equal to 4 on it's third invocation: ");
        baz(); baz();
        assert_eq!(baz(), 4);
        serial_println!("[ok]");
    }
}
 */
