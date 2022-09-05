#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::{prelude::*, CStr16};

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    macro_rules! print {
        ($text: expr) => {
            let mut text_buffer = [0; $text.len() + 1];
            let buf = CStr16::from_str_with_buf($text, &mut text_buffer).unwrap();
            system_table.stdout().output_string(&buf).unwrap();
        };
    }

    macro_rules! clear {
        () => {
            system_table.stdout().clear().unwrap();
        };
    }

    clear!();
    print!("Test");

    loop {}

    Status::SUCCESS
}
