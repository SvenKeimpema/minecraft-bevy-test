use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, EnumWindows, GetWindowTextW};
use windows::core::PCSTR;
use std::ffi::CString;

struct WindowInfo {
    title: String,
    handle: HWND,
}

unsafe extern "system" fn enum_window_callback(window: HWND, lparam: LPARAM) -> BOOL {
    let mut title: [u16; 512] = [0; 512];
    let len = GetWindowTextW(window, &mut title);
    if len > 0 {
        let window_title = String::from_utf16_lossy(&title[..len as usize]);
        let info_ptr = lparam.0 as *mut Vec<WindowInfo>;

        (*info_ptr).push(WindowInfo {
            title: window_title,
            handle: window,
        });
    }
    true.into()
}

fn find_window_by_name(name: &str) -> Option<HWND> {
    unsafe {
        let name = CString::new(name).unwrap();
        let name_ptr = PCSTR::from_raw(name.as_ptr() as *const u8);
        let handle = FindWindowA(None, name_ptr);
        if handle.0 != 0 {
            return Some(handle);
        }
    }

    let mut windows: Vec<WindowInfo> = Vec::new();

    unsafe {
        EnumWindows(
            Some(enum_window_callback),
            LPARAM(&mut windows as *mut _ as isize)
        );
    }

    windows.into_iter()
        .find(|w| w.title.contains(name))
        .map(|w| w.handle)
}

fn main() {
    let window_name = "Minecraft"; 
    match find_window_by_name(window_name) {
        Some(handle) => println!("Found window handle: {:?}", handle),
        None => println!("Window not found"),
    }
}