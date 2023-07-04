const REG_GPIO_MODE1: *mut u32 = 0xf0002800u32 as _;
const REG_GPIO_MODE0: *mut u32 = 0xf0002804u32 as _;
const REG_GPIO_IEN: *mut u32 = 0xf0002808u32 as _;
const REG_GPIO_OEB: *mut u32 = 0xf000280cu32 as _;
#[allow(dead_code)]
const REG_GPIO_IN: *mut u32 = 0xf0002810u32 as _;
const REG_GPIO_OUT: *mut u32 = 0xf0002814u32 as _;

pub fn init() {
    unsafe {
        core::ptr::write_volatile(REG_GPIO_MODE1 as *mut u32, 1);
        core::ptr::write_volatile(REG_GPIO_MODE0 as *mut u32, 0);
        core::ptr::write_volatile(REG_GPIO_IEN as *mut u32, 1);
        core::ptr::write_volatile(REG_GPIO_OEB as *mut u32, 0);
    }
}

pub fn write(value: u32) {
    unsafe {
        core::ptr::write_volatile(REG_GPIO_OUT, value);
    }
}
