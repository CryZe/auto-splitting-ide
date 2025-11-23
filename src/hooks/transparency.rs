#[cfg(windows)]
pub fn use_transparency() -> bool {
    use std::{ffi::c_void, sync::OnceLock};

    use dioxus::{
        core::use_hook,
        desktop::{
            tao::{event::Event, platform::windows::WindowExtWindows},
            use_wry_event_handler, window,
        },
    };
    use windows_sys::Win32::{
        Foundation::{HWND, RECT},
        Graphics::Gdi::{CreateSolidBrush, DeleteObject, FillRect, GetDC, ReleaseDC},
    };

    let is_transparent =
        use_hook(|| window_vibrancy::apply_mica(&window().window, Some(true)).is_ok());

    use_wry_event_handler(move |event, _| {
        if !is_transparent {
            return;
        }

        if let Event::RedrawRequested(_) = event {
            struct GdiInfo(HWND, *mut c_void);
            impl Drop for GdiInfo {
                fn drop(&mut self) {
                    unsafe {
                        DeleteObject(self.1);
                        ReleaseDC(self.0, self.1 as _);
                    }
                }
            }
            unsafe impl Sync for GdiInfo {}
            unsafe impl Send for GdiInfo {}

            static GDI_INFO: OnceLock<GdiInfo> = OnceLock::new();
            let &GdiInfo(hdc, brush) = GDI_INFO.get_or_init(|| {
                let hwnd = window().hwnd();
                let hdc = unsafe { GetDC(hwnd as _) };
                let brush = unsafe { CreateSolidBrush(0x00000000) };
                GdiInfo(hdc, brush)
            });
            let size = window().inner_size();
            let rect = RECT {
                left: 0,
                top: 0,
                right: size.width as _,
                bottom: size.height as _,
            };
            unsafe {
                FillRect(hdc, &rect, brush);
            }
        }
    });

    is_transparent
}

#[cfg(not(windows))]
pub fn use_transparency() -> bool {
    false
}
