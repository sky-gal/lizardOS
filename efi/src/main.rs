#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::{prelude::*, CStr16};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let mut buf = [0; 16];
    let buf = CStr16::from_str_with_buf("Hello, World!", &mut buf).unwrap();
    system_table.stdout().clear().unwrap();
    system_table.stdout().output_string(&buf).unwrap();
    loop {}

    Status::SUCCESS
}
