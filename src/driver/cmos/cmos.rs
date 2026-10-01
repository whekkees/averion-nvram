use crate::arch::io::outb;
use crate::arch::io::inb;

const CMOS_REGISTER_CHOOSE : u16 = 0x70;
const CMOS_REGISTER_DATA : u16 = 0x71;


pub unsafe fn cmos_register_read(register : u8) ->  u8 { 
    outb(CMOS_REGISTER_CHOOSE, register);
    inb(CMOS_REGISTER_DATA)
}

pub unsafe fn cmos_register_write(register: u8, val: u8) {
    outb(CMOS_REGISTER_CHOOSE, register);
    outb(CMOS_REGISTER_DATA, val);
}