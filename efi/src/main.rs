#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use core::convert::TryFrom;

use uefi::{prelude::*, proto::console::text::Key, CStr16, Char16};

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

    macro_rules! println {
        () => {
            print!("\x0a\x0d")
        };
        ($text: expr) => {
            print!($text);
            print!("\x0a\x0d")
        };
    }

    macro_rules! clear {
        () => {
            system_table.stdout().clear().unwrap();
        };
    }

    clear!();
    println!("Starting LizardOS");

    loop {
        print!(">");
        loop {
            let key = system_table.stdin().read_key().unwrap();
            if let Some(Key::Printable(key)) = key {
                if key == Char16::try_from(0xd).unwrap() {
                    break;
                } else {
                    let chars = [key.into(), 0];
                    let string = CStr16::from_u16_with_nul(&chars).unwrap();
                    system_table.stdout().output_string(string).unwrap();
                }
            }
        }
        println!();
    }
}
