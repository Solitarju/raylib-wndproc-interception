use raylib::prelude::*;
use std::mem::transmute;
use winapi::shared::windowsx::*;
use windows::{Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*};

static mut PREV_WNDPROC: WNDPROC = None;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .vsync()
        .undecorated()
        .resizable()
        .build();

    unsafe {
        let result = SetWindowLongPtrW(
            HWND(rl.get_window_handle() as _),
            GWLP_WNDPROC,
            wnd_proc as isize,
        );
        let prev_wnd_proc = transmute::<isize, WNDPROC>(result);
        PREV_WNDPROC = prev_wnd_proc;
    } // Replace raylib window procedure with our own (interception), and store the pointer to the original raylib procedure function so we can keep raylib behaviour.

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_rectangle(0, 0, d.get_screen_width(), 20, Color::GRAY);
    }
}

extern "system" fn wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_NCHITTEST => {
                let mut hit = DefWindowProcW(window, message, wparam, lparam);
                if hit == LRESULT(HTCLIENT.try_into().unwrap()) {
                    hit = LRESULT(HTCAPTION.try_into().unwrap())
                };

                let mouse_global = lparam.0;
                let mouse_x = GET_X_LPARAM(mouse_global);
                let mouse_y = GET_Y_LPARAM(mouse_global); // Mouse positions in relation to system screens.

                let mut rect_mouse_point = POINT {
                    x: mouse_x,
                    y: mouse_y,
                };

                ScreenToClient(window, &mut rect_mouse_point);

                let rect_mouse_x = rect_mouse_point.x;
                let rect_mouse_y = rect_mouse_point.y; // Mouse position in relation to window origin.

                let mut window_rect = RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                }; // window position and size

                let _ = GetWindowRect(window, &mut window_rect).unwrap();
                let window_width = window_rect.right - window_rect.left;
                let _window_height = window_rect.top - window_rect.bottom;

                println!("MOUSE_X: {}, MOUSE_Y: {}", rect_mouse_x, rect_mouse_y);

                if rect_mouse_x > window_width - 4 {
                    CallWindowProcW(PREV_WNDPROC, window, message, wparam, lparam); // Pass procedure to actual procedure function.
                    return LRESULT(11); // HTRIGHT message (https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
                } // This should re-implement the right-side border dragging & resizing, however it doesn't work due to raylib configuration.

                if rect_mouse_y <= 20 && rect_mouse_y > 0 {
                    CallWindowProcW(PREV_WNDPROC, window, message, wparam, lparam);
                    return hit;
                } // Detect if mouse is holding custom title bar, which covers 20px of the top of the program.

                CallWindowProcW(PREV_WNDPROC, window, message, wparam, lparam) // Pass unhandled procedure messages to actual procedure function.
            }
            _ => CallWindowProcW(PREV_WNDPROC, window, message, wparam, lparam),
        }
    }
}
