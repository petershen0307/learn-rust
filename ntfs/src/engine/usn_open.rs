use windows::Win32::Storage::FileSystem::*;
use std::ptr::null_mut;
use windows::core::HSTRING;

fn usn_open(drive_letter: char) -> Option<crate::engine::usn::USN> {
    let u8_letter = drive_letter as u8;
    if u8_letter < b'A' || u8_letter > b'Z'{
        return None;
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
            windows::Win32::Foundation::HANDLE::default()
        )
    };
    Some(eusn)
}
