use std::{
    borrow::Cow,
    ffi::c_void,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    thread,
};

use dioxus::{
    desktop::{
        tao::{platform::windows::WindowExtWindows, window::WindowSizeConstraints},
        use_wry_event_handler, window,
        wry::dpi::PixelUnit,
        Config, LogicalSize, WindowBuilder,
    },
    prelude::*,
};
use dioxus_material_icons::MaterialIconStylesheet;
use indexmap::IndexMap;
use livesplit_auto_splitting::{
    settings::{self, Widget},
    time, TimerState,
};
use time::UtcOffset;
use window_vibrancy::apply_mica;
use windows_sys::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{CreateSolidBrush, DeleteObject, FillRect, GetDC, ReleaseDC},
};

mod runtime_thread;
mod timer;
mod ui;

use timer::*;
use ui::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

static UTC_OFFSET: OnceLock<time::UtcOffset> = OnceLock::new();

struct Widgets(Arc<Vec<Widget>>);

fn main() {
    UTC_OFFSET
        .set(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC))
        .unwrap();

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_transparent(true)
                    .with_inner_size_constraints(WindowSizeConstraints::new(
                        Some(PixelUnit::Logical(700.0.into())),
                        Some(PixelUnit::Logical(500.0.into())),
                        None,
                        None,
                    ))
                    .with_inner_size(LogicalSize::new(800, 600)),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    #[cfg(windows)]
    use_hook(|| {
        // apply_blur(&window().window, None);
        // apply_acrylic(&window().window, Some((0xFF, 0x0, 0x0, 0xFF)));
        apply_mica(&window().window, None).unwrap();
    });

    #[cfg(windows)]
    use_wry_event_handler(|event, _| {
        use dioxus::desktop::tao::event::Event;

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

            let GDI_INFO: OnceLock<GdiInfo> = OnceLock::new();
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

    let wasm_path = use_signal_sync(|| None::<PathBuf>);
    let logs = use_signal_sync(LogEntries::new);
    let timer_state = use_signal_sync(|| TimerState::NotRunning);
    let split_index = use_signal_sync(|| 0);
    let game_time = use_signal_sync(|| time::Duration::ZERO);
    let game_time_state = use_signal_sync(|| GameTimeState::NotInitialized);
    let variables = use_signal_sync(IndexMap::new);
    let processes = use_signal_sync(Vec::new);
    let settings_widgets = use_signal_sync(|| Widgets(Arc::new(Vec::new())));
    let settings_map = use_signal_sync(settings::Map::new);
    let statistics = use_signal_sync(StatisticsData::default);
    let timer = use_signal_sync(|| IdeTimer {
        split_index,
        timer_state,
        game_time,
        game_time_state,
        variables,
        processes,
        settings_widgets,
        settings_map,
        logs,
        wasm_path,
        statistics,
    });
    // TODO: CLI Args
    let optimize = use_signal(|| true);
    let runtime = use_signal_sync(|| build_runtime(optimize()));
    let module = use_signal_sync(|| None);
    let auto_splitter = use_signal_sync(|| None);

    use_hook(move || {
        thread::spawn(move || {
            runtime_thread::run(auto_splitter, timer);
        });
    });

    let mut left_divider = use_signal(|| DividerState {
        size: 250.0,
        is_dragging: false,
        positive: true,
        vertical: false,
        min: 200.0,
        drag_start: 0.0,
    });
    let mut right_divider = use_signal(|| DividerState {
        size: 250.0,
        is_dragging: false,
        positive: false,
        vertical: false,
        min: 200.0,
        drag_start: 0.0,
    });
    let mut bottom_divider = use_signal(|| DividerState {
        size: 250.0,
        is_dragging: false,
        positive: false,
        vertical: true,
        min: 75.0,
        drag_start: 0.0,
    });

    let title = use_memo(move || {
        let mut title = Cow::Borrowed("Auto Splitting IDE");
        if let Some(wasm_path) = wasm_path.read().as_ref().and_then(|p| p.file_name()) {
            use std::fmt::Write;
            let _ = write!(title.to_mut(), " - {}", Path::new(wasm_path).display());
        }
        title
    });

    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Title { "{title}" }
        MaterialIconStylesheet {}

        div {
            class: "app",
            onmousemove: move |ev| {
                for divider in &mut [
                    &mut left_divider,
                    &mut right_divider,
                    &mut bottom_divider,
                ] {
                    let is_dragging = divider.read().is_dragging;
                    if is_dragging {
                        let state = &mut *divider.write();
                        let mouse_coords = ev.client_coordinates();
                        let coord = if state.vertical { mouse_coords.y } else { mouse_coords.x };
                        let mut diff = coord - state.drag_start;
                        if !state.positive {
                            diff = -diff;
                        }
                        state.size = (state.size + diff).max(state.min);
                        state.drag_start = coord;
                    }
                }
            },
            onmouseup: move |_| {
                for divider in &mut [
                    &mut left_divider,
                    &mut right_divider,
                    &mut bottom_divider,
                ] {
                    let is_dragging = divider.read().is_dragging;
                    if is_dragging {
                        let state = &mut *divider.write();
                        state.is_dragging = false;
                    }
                }
            },

            div {
                display: "flex",
                width: "100%",
                height: "100%",
                gap: "10px",

                LeftSideBar {
                    timer,
                    split_index,
                    timer_state,
                    game_time,
                    game_time_state,
                    runtime,
                    module,
                    auto_splitter,
                    statistics,
                    optimize,
                    width: left_divider.read().size,
                }
                VerticalDivider { state: left_divider }
                CentralPanel { logs, bottom_divider }
                VerticalDivider { state: right_divider }
                RightSideBar {
                    auto_splitter,
                    variables,
                    processes,
                    settings_widgets,
                    settings_map,
                    width: right_divider.read().size,
                }
            }
        }
    }
}
