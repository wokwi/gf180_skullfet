const REG_TIMER0_LOAD: *mut u32 = 0xf0005000u32 as _;
#[allow(dead_code)]
const REG_TIMER0_RELOAD: *mut u32 = 0xf0005004u32 as _;
const REG_TIMER0_EN: *mut u32 = 0xf0005008u32 as _;
const REG_TIMER0_UPDATE_VALUE: *mut u32 = 0xf000500cu32 as _;
const REG_TIMER0_VALUE: *mut u32 = 0xf0005010u32 as _;
#[allow(dead_code)]
const REG_TIMER0_EV_STATUS: *mut u32 = 0xf0005014u32 as _;
#[allow(dead_code)]
const REG_TIMER0_EV_PENDING: *mut u32 = 0xf0005018u32 as _;
#[allow(dead_code)]
const REG_TIMER0_EV_ENABLE: *mut u32 = 0xf000501cu32 as _;

pub fn wait(count: u32) {
    unsafe {
        /* Configure timer for a single-shot countdown */
        core::ptr::write_volatile(REG_TIMER0_EN, 0);
        core::ptr::write_volatile(REG_TIMER0_LOAD, count);
        core::ptr::write_volatile(REG_TIMER0_EN, 1);

        // Loop, waiting for value to reach zero
        core::ptr::write_volatile(REG_TIMER0_UPDATE_VALUE, 1); // latch current value
        while core::ptr::read_volatile(REG_TIMER0_VALUE) > 0 {
            core::ptr::write_volatile(REG_TIMER0_UPDATE_VALUE, 1); // latch current value
        }
    }
}
