use super::mprj;

const REG_UART_RXTX: *mut u32 = 0xf0005800u32 as _;
const REG_UART_TXFULL: *mut u32 = 0xf0005804u32 as _;
const REG_UART_RXEMPTY: *mut u32 = 0xf0005808u32 as _;
const REG_UART_EV_STATUS: *mut u32 = 0xf000580cu32 as _;
const UART_EV_STATUS_TX_OFFSET: u32 = 0;
const UART_EV_STATUS_RX_OFFSET: u32 = 1;
const REG_UART_EV_PENDING: *mut u32 = 0xf0005810u32 as _;
const UART_EV_PENDING_TX_OFFSET: u32 = 0;
const UART_EV_PENDING_RX_OFFSET: u32 = 1;
const REG_UART_EV_ENABLE: *mut u32 = 0xf0005814u32 as _;
const UART_EV_ENABLE_TX_OFFSET: u32 = 0;
const UART_EV_ENABLE_RX_OFFSET: u32 = 1;
const REG_UART_TXEMPTY: *mut u32 = 0xf0005818u32 as _;
const REG_UART_RXFULL: *mut u32 = 0xf000581cu32 as _;
const UART_ENABLED_BASE: *mut u32 = 0xf0006000u32 as _;
const REG_UART_ENABLED_OUT: *mut u32 = 0xf0006000u32 as _;

pub fn init() {
    unsafe {
        mprj::set_io_mode(5, mprj::GPIO_MODE_MGMT_STD_INPUT_NOPULL); // UART Rx
        mprj::set_io_mode(6, mprj::GPIO_MODE_MGMT_STD_OUTPUT); // UART Tx

        core::ptr::write_volatile(REG_UART_ENABLED_OUT, 1);
    }
}

pub fn write(value: u8) {
    unsafe {
        while (core::ptr::read_volatile(REG_UART_TXFULL) & 1) != 0 { /* wait */ }
        core::ptr::write_volatile(REG_UART_RXTX, value.into());
    }
}

pub fn write_str(value: &str) {
    for c in value.bytes() {
        write(c);
    }
}
