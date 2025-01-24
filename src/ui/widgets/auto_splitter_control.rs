use dioxus::prelude::*;
use livesplit_auto_splitting::{AutoSplitter, CompiledAutoSplitter, Runtime};
use notify::{EventKind, RecursiveMode, Watcher};

use crate::{build_runtime, ui::Toggle, IdeTimer};

use super::Widget;

#[component]
pub fn AutoSplitterControl(
    timer: SyncSignal<IdeTimer>,
    runtime: SyncSignal<Runtime>,
    module: SyncSignal<Option<CompiledAutoSplitter>>,
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    mut optimize: Signal<bool>,
) -> Element {
    let open = move |_| async move {
        let Some(file) = rfd::AsyncFileDialog::new()
            .add_filter("WebAssembly Files", &["wasm"])
            .add_filter("All Files", &["*"])
            .pick_file()
            .await
        else {
            return;
        };

        timer
            .read()
            .load_file(file.path(), runtime, module, auto_splitter);
    };

    let wasm_path = timer.read().wasm_path;

    use_memo(move || {
        struct NotEq<T>(T);

        impl<T> PartialEq for NotEq<T> {
            fn eq(&self, _: &Self) -> bool {
                false
            }
        }

        let wasm_path = wasm_path.read();
        let wasm_path = wasm_path.as_ref()?;

        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
                let Ok(event) = event else {
                    return;
                };
                let should_reload = match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => true,
                    _ => event.need_rescan(),
                };
                if should_reload {
                    timer.read().reload(runtime, module, auto_splitter);
                }
            })
            .ok()?;
        watcher.watch(wasm_path, RecursiveMode::NonRecursive).ok()?;
        Some(NotEq(watcher))
    });

    let has_auto_splitter = auto_splitter.read().is_some();

    rsx! {
        Widget { title: "Auto Splitter",
            button { onclick: open, "Open" }
            if has_auto_splitter {
                button { onclick: move |_| timer.read().restart(runtime, module, auto_splitter),
                    "Restart"
                }
                button {
                    class: "stop",
                    onclick: move |_| {
                        if let Some(auto_splitter) = &*auto_splitter.read() {
                            auto_splitter.interrupt_handle().interrupt();
                        }
                    },
                    "Kill"
                }
                Toggle {
                    centered: true,
                    checked: optimize(),
                    onchange: move |event: Event<FormData>| {
                        let should_optimize = event.checked();
                        optimize.set(should_optimize);
                        runtime.set(build_runtime(should_optimize));
                        timer.read().reload(runtime, module, auto_splitter);
                    },
                    "Optimize"
                }
            }
        }
    }
}
