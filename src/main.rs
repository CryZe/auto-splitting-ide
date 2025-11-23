#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, OnceLock},
    thread,
};

use dioxus::{
    desktop::{
        tao::window::WindowSizeConstraints, wry::dpi::PixelUnit, Config, LogicalSize, WindowBuilder,
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

mod hooks;
mod runtime_thread;
mod timer;
mod ui;

use hooks::use_transparency;
use timer::*;
use ui::*;

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
    let is_transparent = use_transparency();
    let wasm_path = use_signal_sync(|| None::<PathBuf>);
    let logs = use_signal_sync(LogEntries::new);
    let timer_state = use_signal_sync(|| TimerState::NotRunning);
    let split_index = use_signal_sync(|| 0);
    let segment_splitted = use_signal_sync(Vec::new);
    let game_time = use_signal_sync(|| time::Duration::ZERO);
    let game_time_state = use_signal_sync(|| GameTimeState::NotInitialized);
    let variables = use_signal_sync(IndexMap::new);
    let processes = use_signal_sync(Vec::new);
    let settings_widgets = use_signal_sync(|| Widgets(Arc::new(Vec::new())));
    let settings_map = use_signal_sync(settings::Map::new);
    let statistics = use_signal_sync(StatisticsData::default);
    let timer = use_signal_sync(|| IdeTimer {
        split_index,
        segment_splitted,
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
        let thread = thread::spawn(move || {
            runtime_thread::run(auto_splitter, timer);
        });
        struct ThreadJoiner(
            Option<thread::JoinHandle<()>>,
            SyncSignal<Option<livesplit_auto_splitting::AutoSplitter<IdeTimer>>>,
        );
        impl Drop for ThreadJoiner {
            fn drop(&mut self) {
                if let Some(auto_splitter) = &*self.1.read() {
                    auto_splitter.interrupt_handle().interrupt();
                }
                let join_handle = self.0.take().unwrap();
                runtime_thread::RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
                let _ = join_handle.join();
            }
        }
        Rc::new(ThreadJoiner(Some(thread), auto_splitter))
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

    const MAIN_CSS_INLINE: &str = include_str!("../assets/main.css");

    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Style { "{MAIN_CSS_INLINE}" }
        if is_transparent {
            document::Style { ":root {{ background: transparent; }}" }
        }
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

            ui::swapy::Container {
                id: "swapy",
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
