#![windows_subsystem = "windows"] // Prevents console window from appearing

use std::ptr;
use windows::core::{HSTRING, Result, PCSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM, POINT};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Controls::{BS_PUSHBUTTON, CreateWindowExA, CreateWindowExW, BUTTON_STYLE, WS_CHILD, WS_VISIBLE};
use windows::Win32::UI::WindowsAndMessaging::{
    DefWindowProcA, DispatchMessageA, GetMessageA, MessageBoxA, PostQuitMessage, RegisterClassA,
    TranslateMessage, CW_USEDEFAULT, MSG, WNDCLASSA, WM_COMMAND, WM_CREATE, WM_DESTROY,
    WS_OVERLAPPEDWINDOW, BN_CLICKED,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(PCSTR(ptr::null_mut()))?;
        let class_name = PCSTR("SampleWindowClass\0".as_ptr());

        let wc = WNDCLASSA {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: class_name,
            ..Default::default()
        };

        if RegisterClassA(&wc) == 0 {
            return Err(windows::core::Error::from_win32());
        }

        let hwnd = CreateWindowExA(
            Default::default(),
            class_name,
            PCSTR("Win32 GUI Demo\0".as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            320,
            240,
            HWND(ptr::null_mut()),
            Default::default(),
            instance,
            ptr::null_mut(),
        );

        if hwnd.0 == 0 {
            return Err(windows::core::Error::from_win32());
        }

        windows::Win32::UI::WindowsAndMessaging::ShowWindow(hwnd, windows::Win32::UI::WindowsAndMessaging::SW_SHOW);

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, HWND(ptr::null_mut()), 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        Ok(())
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut BUTTON: HWND = HWND(0);

    match msg {
        WM_CREATE => {
            BUTTON = CreateWindowExA(
                Default::default(),
                PCSTR("BUTTON\0".as_ptr()),
                PCSTR("Click Me\0".as_ptr()),
                WS_VISIBLE | WS_CHILD | BS_PUSHBUTTON,
                100,
                100,
                100,
                30,
                hwnd,
                1 as windows::Win32::UI::WindowsAndMessaging::HMENU,
                windows::Win32::System::LibraryLoader::GetModuleHandleA(PCSTR(ptr::null_mut())).unwrap(),
                ptr::null_mut(),
            );
            if BUTTON.0 == 0 {
                return LRESULT(-1);
            }

            LRESULT(0)
        }
        WM_COMMAND => {
            if wparam.0 as u32 & 0xFFFF == 1 && (wparam.0 >> 16) as u32 == BN_CLICKED.0 {
                on_button_click(hwnd);
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

unsafe fn on_button_click(hwnd: HWND) {
    MessageBoxA(
        hwnd,
        PCSTR("Hello, Win32!\0".as_ptr()),
        PCSTR("Greeting\0".as_ptr()),
        windows::Win32::UI::WindowsAndMessaging::MB_OK | windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION,
    );
}