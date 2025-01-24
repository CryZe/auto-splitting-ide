use std::{fs, path::PathBuf};

use dioxus::prelude::*;
use document::eval;

const MONACO_FOLDER: Asset = asset!("/assets/monaco/min/vs");

#[component]
pub fn CodeEditor(text: Signal<String>, path: Signal<PathBuf>) -> Element {
    // TODO: build.rs for downloading monaco:
    // https://github.com/DioxusLabs/dioxus/blob/041c570efea755d6ab77d782ae49e236a666cf18/packages/playwright-tests/cli-optimization/build.rs

    use_future(move || async move {
        let mut eval = eval(include_str!("code_editor.js"));

        eval.send(MONACO_FOLDER.to_string()).unwrap();
        eval.send(&*text.read()).unwrap();

        while let Ok(text) = eval.recv::<String>().await {
            // TODO: Error handling
            fs::write(&*path.read(), text).unwrap();
        }
    });

    rsx! {
        div { flex_grow: "1", border: "1px solid #ffffff20", class: "monaco" }
    }
}
