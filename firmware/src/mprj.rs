pub const GPIO_MODE_MGMT_STD_INPUT_NOPULL: u32 = 0x007;
pub const GPIO_MODE_MGMT_STD_INPUT_PULLDOWN: u32 = 0x047;
pub const GPIO_MODE_MGMT_STD_INPUT_PULLUP: u32 = 0x087;
pub const GPIO_MODE_MGMT_STD_OUTPUT: u32 = 0x00b;
pub const GPIO_MODE_MGMT_STD_BIDIRECTIONAL: u32 = 0x009;
pub const GPIO_MODE_MGMT_STD_OUT_MONITORED: u32 = 0x00F;

pub const GPIO_MODE_USER_STD_INPUT_NOPULL: u32 = 0x006;
pub const GPIO_MODE_USER_STD_INPUT_PULLDOWN: u32 = 0x046;
pub const GPIO_MODE_USER_STD_INPUT_PULLUP: u32 = 0x086;
pub const GPIO_MODE_USER_STD_OUTPUT: u32 = 0x00a;
pub const GPIO_MODE_USER_STD_BIDIRECTIONAL: u32 = 0x00C;
pub const GPIO_MODE_USER_STD_OUT_MONITORED: u32 = 0x00E;

const REG_MPRJ_XFER: *mut u32 = 0x26000000u32 as _;
const REG_MPRJ_PWR: *mut u32 = 0x26000004u32 as _;
const REG_MPRJ_IRQ: *mut u32 = 0x26100014u32 as _;
const REG_MPRJ_DATAL: *mut u32 = 0x2600000cu32 as _;
const REG_MPRJ_DATAH: *mut u32 = 0x26000010u32 as _;

const REG_MPRJ_IO_0: *mut u32 = 0x26000024u32 as _;
const REG_MPRJ_IO_1: *mut u32 = 0x26000028u32 as _;
const REG_MPRJ_IO_2: *mut u32 = 0x2600002cu32 as _;
const REG_MPRJ_IO_3: *mut u32 = 0x26000030u32 as _;
const REG_MPRJ_IO_4: *mut u32 = 0x26000034u32 as _;
const REG_MPRJ_IO_5: *mut u32 = 0x26000038u32 as _;
const REG_MPRJ_IO_6: *mut u32 = 0x2600003cu32 as _;

const REG_MPRJ_IO_7: *mut u32 = 0x26000040u32 as _;
const REG_MPRJ_IO_8: *mut u32 = 0x26000044u32 as _;
const REG_MPRJ_IO_9: *mut u32 = 0x26000048u32 as _;
const REG_MPRJ_IO_10: *mut u32 = 0x2600004cu32 as _;

const REG_MPRJ_IO_11: *mut u32 = 0x26000050u32 as _;
const REG_MPRJ_IO_12: *mut u32 = 0x26000054u32 as _;
const REG_MPRJ_IO_13: *mut u32 = 0x26000058u32 as _;
const REG_MPRJ_IO_14: *mut u32 = 0x2600005cu32 as _;

const REG_MPRJ_IO_15: *mut u32 = 0x26000060u32 as _;
const REG_MPRJ_IO_16: *mut u32 = 0x26000064u32 as _;
const REG_MPRJ_IO_17: *mut u32 = 0x26000068u32 as _;
const REG_MPRJ_IO_18: *mut u32 = 0x2600006cu32 as _;

const REG_MPRJ_IO_19: *mut u32 = 0x26000070u32 as _;
const REG_MPRJ_IO_20: *mut u32 = 0x26000074u32 as _;
const REG_MPRJ_IO_21: *mut u32 = 0x26000078u32 as _;
const REG_MPRJ_IO_22: *mut u32 = 0x2600007cu32 as _;

const REG_MPRJ_IO_23: *mut u32 = 0x26000080u32 as _;
const REG_MPRJ_IO_24: *mut u32 = 0x26000084u32 as _;
const REG_MPRJ_IO_25: *mut u32 = 0x26000088u32 as _;
const REG_MPRJ_IO_26: *mut u32 = 0x2600008cu32 as _;

const REG_MPRJ_IO_27: *mut u32 = 0x26000090u32 as _;
const REG_MPRJ_IO_28: *mut u32 = 0x26000094u32 as _;
const REG_MPRJ_IO_29: *mut u32 = 0x26000098u32 as _;
const REG_MPRJ_IO_30: *mut u32 = 0x2600009cu32 as _;
const REG_MPRJ_IO_31: *mut u32 = 0x260000a0u32 as _;

const REG_MPRJ_IO_32: *mut u32 = 0x260000a4u32 as _;
const REG_MPRJ_IO_33: *mut u32 = 0x260000a8u32 as _;
const REG_MPRJ_IO_34: *mut u32 = 0x260000acu32 as _;
const REG_MPRJ_IO_35: *mut u32 = 0x260000b0u32 as _;
const REG_MPRJ_IO_36: *mut u32 = 0x260000b4u32 as _;
const REG_MPRJ_IO_37: *mut u32 = 0x260000b8u32 as _;

/* An vector with all MPRJ IOS: */
const REG_MPRJ_IOS: [*mut u32; 38] = [
    REG_MPRJ_IO_0,
    REG_MPRJ_IO_1,
    REG_MPRJ_IO_2,
    REG_MPRJ_IO_3,
    REG_MPRJ_IO_4,
    REG_MPRJ_IO_5,
    REG_MPRJ_IO_6,
    REG_MPRJ_IO_7,
    REG_MPRJ_IO_8,
    REG_MPRJ_IO_9,
    REG_MPRJ_IO_10,
    REG_MPRJ_IO_11,
    REG_MPRJ_IO_12,
    REG_MPRJ_IO_13,
    REG_MPRJ_IO_14,
    REG_MPRJ_IO_15,
    REG_MPRJ_IO_16,
    REG_MPRJ_IO_17,
    REG_MPRJ_IO_18,
    REG_MPRJ_IO_19,
    REG_MPRJ_IO_20,
    REG_MPRJ_IO_21,
    REG_MPRJ_IO_22,
    REG_MPRJ_IO_23,
    REG_MPRJ_IO_24,
    REG_MPRJ_IO_25,
    REG_MPRJ_IO_26,
    REG_MPRJ_IO_27,
    REG_MPRJ_IO_28,
    REG_MPRJ_IO_29,
    REG_MPRJ_IO_30,
    REG_MPRJ_IO_31,
    REG_MPRJ_IO_32,
    REG_MPRJ_IO_33,
    REG_MPRJ_IO_34,
    REG_MPRJ_IO_35,
    REG_MPRJ_IO_36,
    REG_MPRJ_IO_37,
];

pub fn set_io_mode(index: u32, mode: u32) {
    unsafe {
        core::ptr::write_volatile(REG_MPRJ_IOS[index as usize], mode);
    }
}

fn io_reg(index: u32) -> *mut u32 {
    if index >= 32 {
        return REG_MPRJ_DATAH;
    } else {
        return REG_MPRJ_DATAL;
    }
}

pub fn io_read(index: u32) -> u32 {
    let shift = index & 0x1f;
    unsafe {
        return (core::ptr::read_volatile(io_reg(index)) >> shift) & 1;
    }
}

static mut IO_OUT_VALUES: [u32; 2] = [0u32; 2];

pub fn io_write(index: u32, value: u32) {
    let reg: *mut u32 = io_reg(index);
    let mask: u32 = 1 << (index & 0x1f);
    unsafe {
        let current_value: u32 = IO_OUT_VALUES[(index >> 5) as usize];
        let new_value: u32 = if value != 0 {
            current_value | mask
        } else {
            current_value & !mask
        };
        IO_OUT_VALUES[(index >> 5) as usize] = new_value;
        core::ptr::write_volatile(reg, new_value);
    }
}

pub fn commit() {
    unsafe {
        core::ptr::write_volatile(REG_MPRJ_XFER, 1);
        while core::ptr::read_volatile(REG_MPRJ_XFER) == 1 { /* busy wait */ }
    }
}
