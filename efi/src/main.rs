#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::{
    data_types::Identify,
    prelude::*,
    proto::console::serial::Serial,
    table::boot::{OpenProtocolAttributes, OpenProtocolParams},
    CStr16,
};

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let serial = system_table.boot_services().open_protocol::<Serial>(
        OpenProtocolParams {
            handle: system_table
                .boot_services()
                .locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(&Serial::GUID))
                .unwrap()
                .handles()[0],
            agent: handle,
            controller: None,
        },
        OpenProtocolAttributes::GetProtocol,
    ).unwrap().interface.get();

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

    unsafe { Serial::write(&mut *serial, "serial".as_bytes()).unwrap(); }

    loop {}
}
