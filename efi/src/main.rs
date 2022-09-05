#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::{prelude::*, CStr16};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    system_table.stdout().clear().unwrap();

    let mut text_buffer = [0; 80];
    macro_rules! print {
        ($text: expr) => {
            let buf = CStr16::from_str_with_buf($text, &mut text_buffer).unwrap();
            system_table.stdout().output_string(&buf).unwrap();
        };
    }

    print!("Test");

    loop {}

    Status::SUCCESS
}
