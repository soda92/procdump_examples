#![windows_subsystem = "windows"] // Prevents console window from appearing

use std::ptr;
use windows::core::{Result, PCSTR};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, MessageBoxA,
    PostQuitMessage, RegisterClassA, TranslateMessage, BN_CLICKED, CW_USEDEFAULT,
    MSG, WM_COMMAND, WM_CREATE, WM_DESTROY, WNDCLASSA, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(PCSTR(ptr::null_mut()))?;
        let class_name = PCSTR("SampleWindowClass\0".as_ptr());

        let wc = WNDCLASSA {
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
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
            Some(HWND(ptr::null_mut())),
            Default::default(),
            Some(instance.into()),
            Some(ptr::null_mut()),
        );

        if hwnd.is_err() {
            return Err(windows::core::Error::from_win32());
        }

        let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
            hwnd.unwrap(),
            windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
        );

        let mut msg = MSG::default();
        while GetMessageA(&mut msg, Some(HWND(ptr::null_mut())), 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }

        Ok(())
    }
}

use std::ffi::c_void;

fn convert_i32_to_mut_c_void(value: *const i32) -> *mut c_void {
    value as *const i32 as *mut c_void
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut BUTTON: HWND = HWND(std::ptr::null_mut());
    static mut R: i32 = 1;

    match msg {
        WM_CREATE => {
            BUTTON = CreateWindowExA(
                Default::default(),
                PCSTR("BUTTON\0".as_ptr()),
                PCSTR("Click Me\0".as_ptr()),
                WS_VISIBLE | WS_CHILD,
                100,
                100,
                100,
                30,
                Some(hwnd),
                Some(windows::Win32::UI::WindowsAndMessaging::HMENU(convert_i32_to_mut_c_void(&raw const R))),
                Some(windows::Win32::System::LibraryLoader::GetModuleHandleA(PCSTR(ptr::null_mut()))
                    .unwrap().into()),
                Some(ptr::null_mut()),
            ).unwrap();
            // if BUTTON.is == 0 {
            //     return LRESULT(-1);
            // }

            LRESULT(0)
        }
        WM_COMMAND => {
            if wparam.0 as u32 & 0xFFFF == 1 && (wparam.0 >> 16) as u32 == BN_CLICKED {
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
        Some(hwnd),
        PCSTR("Hello, Win32!\0".as_ptr()),
        PCSTR("Greeting\0".as_ptr()),
        windows::Win32::UI::WindowsAndMessaging::MB_OK
            | windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION,
    );
}
