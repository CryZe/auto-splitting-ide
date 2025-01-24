use std::{fs, path::PathBuf};

use dioxus::prelude::*;

use super::CodeEditor;

#[component]
pub fn SourcePanel(show_editor: Signal<bool>) -> Element {
    let mut text = use_signal(String::new);
    let mut path = use_signal(PathBuf::new);

    let open = move |_| async move {
        let Some(file) = rfd::AsyncFileDialog::new().pick_file().await else {
            return;
        };

        path.set(file.path().to_path_buf());

        let Ok(file_text) = fs::read_to_string(file.path()) else {
            return;
        };
        text.set(file_text);
        show_editor.set(true);
    };

    rsx! {
        if *show_editor.read() {
            CodeEditor { text, path }
        } else {
            button { onclick: open, "Open Source File" }
        }
    }
}
