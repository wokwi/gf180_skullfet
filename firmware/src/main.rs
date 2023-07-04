#![no_std]
#![no_main]

extern crate riscv_rt;

use riscv_rt::entry;

mod delay;
mod gpio;
mod mprj;
mod uart;

mod uart_printer;
use uart_printer::Printer;

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    println!("PANIC!!! {}", _info);

    // Blink slowly
    loop {
        gpio::write(0);
        delay::wait(20000000);
        gpio::write(1);
        delay::wait(20000000);
    }
}

#[entry]
fn main() -> ! {
    gpio::init();
    uart::init();

    mprj::set_io_mode(8, mprj::GPIO_MODE_MGMT_STD_OUT_MONITORED); // Inverter input
    mprj::set_io_mode(16, mprj::GPIO_MODE_USER_STD_OUT_MONITORED); // Inverter output

    mprj::set_io_mode(9, mprj::GPIO_MODE_MGMT_STD_OUT_MONITORED); // NAND input A
    mprj::set_io_mode(10, mprj::GPIO_MODE_MGMT_STD_OUT_MONITORED); // NAND input B
    mprj::set_io_mode(17, mprj::GPIO_MODE_USER_STD_OUT_MONITORED); // NAND output

    mprj::commit();

    println!("SkullFET test suite\n");

    println!("1. Inverter test");

    mprj::io_write(8, 0);
    assert_eq!(mprj::io_read(16), 1);

    mprj::io_write(8, 1);
    assert_eq!(mprj::io_read(16), 0);

    mprj::io_write(8, 0);
    assert_eq!(mprj::io_read(16), 1);

    mprj::io_write(8, 1);
    assert_eq!(mprj::io_read(16), 0);

    println!("2. NAND test");

    mprj::io_write(9, 0);
    mprj::io_write(10, 0);
    assert_eq!(mprj::io_read(17), 1);

    mprj::io_write(9, 0);
    mprj::io_write(10, 1);
    assert_eq!(mprj::io_read(17), 1);

    mprj::io_write(9, 1);
    mprj::io_write(10, 0);
    assert_eq!(mprj::io_read(17), 1);

    mprj::io_write(9, 1);
    mprj::io_write(10, 1);
    assert_eq!(mprj::io_read(17), 0);

    println!("Tests passed!");
    println!("Blinking forever...");

    loop {
        gpio::write(0);
        mprj::io_write(8, 0);
        delay::wait(2000000);
        gpio::write(1);
        mprj::io_write(8, 1);
        delay::wait(2000000);
    }
}
