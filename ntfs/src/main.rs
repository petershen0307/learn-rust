mod engine;

use std::ptr::null_mut;
use windows::core::HSTRING;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::IO::*;

fn main() {
    println!("Hello, world!");
    test_to_run_usn();
}

fn test_to_run_usn() {
    let drive_letter = char::from('C');
    let u8_letter = drive_letter as u8;
    if u8_letter < b'A' || u8_letter > b'Z' {
        return;
    }
    let mut eusn = crate::engine::usn::USN::new();
    eusn.drive_letter = drive_letter;
    let volume_path = format!("\\\\?\\{}:", drive_letter);
    let handle = unsafe {
        CreateFileW(
            &HSTRING::from(volume_path.as_str()),
            0,
            FILE_SHARE_READ,
            Some(null_mut()),
            windows::Win32::Storage::FileSystem::OPEN_EXISTING,
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES::default(),
            windows::Win32::Foundation::HANDLE::default(),
        )
    };
    if handle.is_err() {
        return;
    }
    let handle_val = handle.unwrap();
    if handle_val.is_invalid() {
        return;
    }

    // get usn data
    let mut in_data = windows::Win32::System::Ioctl::MFT_ENUM_DATA_V0::default();
    // need to be changed to next start reference
    in_data.StartFileReferenceNumber = 0;
    in_data.LowUsn = 0;
    in_data.HighUsn = 0x7FFFFFFFFFFFFFFF;
    let mut usn_data = vec![0u8; 64 * 1024];
    let mut bytes_returned: u32 = 0;
    let result = unsafe {
        DeviceIoControl(
            handle_val,
            windows::Win32::System::Ioctl::FSCTL_ENUM_USN_DATA,
            Some(&mut in_data as *mut _ as *mut _),
            std::mem::size_of::<windows::Win32::System::Ioctl::MFT_ENUM_DATA_V0>() as u32,
            Some(usn_data.as_mut_ptr() as *mut _),
            usn_data.len() as u32,
            Some(&mut bytes_returned),
            None,
        )
    };

    unsafe {
        CloseHandle(handle_val);
    }
}
