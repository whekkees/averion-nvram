use crate::driver::cmos::cmos::cmos_register_write;
use crate::driver::cmos::cmos::cmos_register_read;

pub unsafe fn write(register: u8, val: u8) { 
    cmos_register_write(register, val)
}

pub unsafe fn read(register: u8) -> u8 { 
    cmos_register_read(register)
}