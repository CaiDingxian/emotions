use std::fmt::Error;
use windows::core::{Type, HSTRING};
use windows::Win32::Foundation::*;
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Ole::CF_HDROP;
use windows::Win32::UI::Shell::*;

pub struct WinClip {}

impl WinClip {
    pub fn clip(url: String) {
        // Get the file path as a HSTRING
        let file_path = HSTRING::from(&url);

        // Open and empty the clipboard
        unsafe {
            OpenClipboard(None).unwrap();
            EmptyClipboard().unwrap();
        }

        // Copy the file path to the clipboard
        unsafe {
            let mut data = DROPFILES {
                pFiles: std::mem::size_of::<DROPFILES>() as u32,
                pt: POINT { x: 0, y: 0 },
                fNC: FALSE,
                fWide: TRUE,
            };
            let size = std::mem::size_of_val(&data) + file_path.len() * 2 + 2;
            let global = GlobalAlloc(GMEM_MOVEABLE, size).unwrap();
            let buffer = GlobalLock(global) as *mut u8;
            std::ptr::copy_nonoverlapping(
                &data as *const _ as *const u8,
                buffer,
                std::mem::size_of_val(&data),
            );
            std::ptr::copy_nonoverlapping(
                file_path.as_wide().as_ptr() as *const u8,
                buffer.add(std::mem::size_of_val(&data)),
                file_path.len() * 2,
            );
            buffer.add(size - 2).write(0);
            buffer.add(size - 1).write(0);
            GlobalUnlock(global);

            let cf_drop = CF_HDROP.0 as u32;

            SetClipboardData(CF_HDROP.0 as u32, HANDLE(global.0)).expect("访问剪贴板失败");
        }

        // Close the clipboard
        unsafe {
            CloseClipboard().unwrap();
        };
    }
}
