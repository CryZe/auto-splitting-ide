use std::{path::PathBuf, sync::Arc};

use dioxus::{desktop::window, prelude::*};
use livesplit_auto_splitting::{
    settings::{self, FileFilter, WidgetKind},
    wasi_path, AutoSplitter,
};

use crate::{
    ui::{Panel, Toggle, Widget},
    IdeTimer, Widgets,
};

#[component]
pub fn SettingsGui(
    settings_widgets: SyncSignal<Widgets>,
    settings_map: SyncSignal<settings::Map>,
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
) -> Element {
    let current_settings_map = settings_map.read();

    rsx! {
        Widget { title: "Settings GUI",
            if !settings_widgets.read().0.is_empty() {
                Panel { display: "flex", flex_direction: "column", gap: "5px",
                    for widget in settings_widgets.read().0.iter() {
                        div { title: if let Some(tooltip) = &widget.tooltip { "{tooltip}" },
                            match &widget.kind {
                                WidgetKind::Title { heading_level } => {
                                    match heading_level {
                                        0 => rsx! {
                                            h2 { display: "inline", "{widget.description}" }
                                        },
                                        1 => rsx! {
                                            h3 { display: "inline", "{widget.description}" }
                                        },
                                        2 => rsx! {
                                            h4 { display: "inline", "{widget.description}" }
                                        },
                                        3 => rsx! {
                                            h5 { display: "inline", "{widget.description}" }
                                        },
                                        4 => rsx! {
                                            h6 { display: "inline", "{widget.description}" }
                                        },
                                        _ => rsx! {
                                        "{widget.description}"
                                        },
                                    }
                                }
                                WidgetKind::Bool { default_value } => {
                                    let key = widget.key.clone();
                                    rsx! {
                                        Toggle {
                                            checked: current_settings_map.get(&key).and_then(|v| v.to_bool()).unwrap_or(*default_value),
                                            onchange: move |cx: Event<FormData>| {
                                                let guard = &*auto_splitter.read();
                                                let Some(auto_splitter) = guard else {
                                                    return;
                                                };
                                                let value = settings::Value::Bool(cx.checked());
                                                loop {
                                                    let old = auto_splitter.settings_map();
                                                    let mut new = old.clone();
                                                    new.insert(key.clone(), value.clone());
                                                    if auto_splitter.set_settings_map_if_unchanged(&old, new) {
                                                        break;
                                                    }
                                                }
                                                settings_map.set(auto_splitter.settings_map());
                                            },
                                            "{widget.description}"
                                        }
                                    }
                                }
                                WidgetKind::Choice { default_option_key, options } => {
                                    let key = widget.key.clone();
                                    let current_value = &**current_settings_map
                                        .get(&key)
                                        .and_then(|v| v.as_string())
                                        .unwrap_or(default_option_key);
                                    rsx! {
                                        div { display: "flex", align_items: "center", gap: "8px",
                                            "{widget.description}"
                                            select {
                                                flex_grow: 1,
                                                onchange: move |cx: Event<FormData>| {
                                                    let guard = &*auto_splitter.read();
                                                    let Some(auto_splitter) = guard else {
                                                        return;
                                                    };
                                                    let value = settings::Value::String(cx.value().into());
                                                    loop {
                                                        let old = auto_splitter.settings_map();
                                                        let mut new = old.clone();
                                                        new.insert(key.clone(), value.clone());
                                                        if auto_splitter.set_settings_map_if_unchanged(&old, new) {
                                                            break;
                                                        }
                                                    }
                                                    settings_map.set(auto_splitter.settings_map());
                                                },
                                                for option in options.iter() {
                                                    option {
                                                        selected: &*option.key == current_value,
                                                        value: "{option.key}",
                                                        "{option.description}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                WidgetKind::FileSelect { filters } => {
                                    let key = widget.key.clone();
                                    rsx! {
                                        FileSelect {
                                            current_path: current_settings_map.get(&key).and_then(|v| v.as_string().cloned()),
                                            filters: Identity(filters.clone()),
                                            onchoose: move |path: PathBuf| {
                                                let guard = &*auto_splitter.read();
                                                let Some(auto_splitter) = guard else {
                                                    return;
                                                };
                                                if let Some(path) = wasi_path::from_native(&path) {
                                                    let value = settings::Value::String(path.into());
                                                    loop {
                                                        let old = auto_splitter.settings_map();
                                                        let mut new = old.clone();
                                                        new.insert(key.clone(), value.clone());
                                                        if auto_splitter.set_settings_map_if_unchanged(&old, new) {
                                                            break;
                                                        }
                                                    }
                                                } else {
                                                    loop {
                                                        let old = auto_splitter.settings_map();
                                                        let mut new = old.clone();
                                                        new.remove(&key);
                                                        if auto_splitter.set_settings_map_if_unchanged(&old, new) {
                                                            break;
                                                        }
                                                    }
                                                }
                                                settings_map.set(auto_splitter.settings_map());
                                            },
                                            "{widget.description}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Identity<T>(Arc<T>);

impl<T> PartialEq for Identity<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(PartialEq)]
enum Extensions {
    Dynamic(Vec<String>),
    Static(&'static [&'static str]),
}

#[component]
fn FileSelect(
    children: Element,
    current_path: Option<Arc<str>>,
    filters: Identity<Vec<FileFilter>>,
    onchoose: EventHandler<PathBuf>,
) -> Element {
    let filters = use_memo(move || {
        let mut result = vec![];
        for filter in filters.0.iter() {
            match filter {
                FileFilter::Name {
                    description,
                    pattern,
                } => {
                    let description = match &description {
                        Some(description) => (**description).to_owned(),
                        None => {
                            let mime = pattern.split(' ').find_map(|pat| {
                                let (name, ext) = pat.rsplit_once('.')?;
                                if name != "*" {
                                    return None;
                                }
                                if ext.contains('*') {
                                    return None;
                                }
                                mime_guess::from_ext(ext).first()
                            });

                            if let Some(mime) = mime {
                                mime_desc(mime.type_().as_str(), mime.subtype().as_str())
                            } else {
                                let mut output = String::new();

                                let mut ext_count = 0;

                                let only_contains_extensions = pattern.split(' ').all(|pat| {
                                    ext_count += 1;
                                    let Some((name, ext)) = pat.rsplit_once('.') else {
                                        return false;
                                    };
                                    name == "*" && !ext.contains('*')
                                });

                                if only_contains_extensions {
                                    for (i, ext) in pattern
                                        .split(' ')
                                        .filter_map(|pat| {
                                            let (_, ext) = pat.rsplit_once('.')?;
                                            Some(ext)
                                        })
                                        .enumerate()
                                    {
                                        if i != 0 {
                                            output.push_str(if i + 1 != ext_count {
                                                ", "
                                            } else {
                                                " or "
                                            });
                                        }

                                        output.extend(ext.chars().flat_map(|c| c.to_uppercase()));
                                    }

                                    output.push_str(" files");
                                } else {
                                    output.push_str(pattern.trim());
                                }

                                output
                            }
                        }
                    };

                    result.push((
                        description,
                        Extensions::Dynamic(
                            pattern
                                .split(' ')
                                .filter_map(|pattern| {
                                    let (_, ext) = pattern.split_once('.')?;
                                    if ext == "*" {
                                        return None;
                                    }
                                    Some(ext.to_owned())
                                })
                                .collect::<Vec<_>>(),
                        ),
                    ));
                }
                FileFilter::MimeType(mime_type) => {
                    let Some((top, sub)) = mime_type.split_once('/') else {
                        continue;
                    };
                    if top == "*" {
                        continue;
                    }
                    let Some(extensions) = mime_guess::get_extensions(top, sub) else {
                        continue;
                    };

                    result.push((mime_desc(top, sub), Extensions::Static(extensions)));
                }
            };
        }
        result
    });

    let onclick = move |_| {
        let current_path = current_path.clone();
        async move {
            let mut dialog = rfd::AsyncFileDialog::new().set_parent(&window().window);
            if let Some(current_path) = current_path.and_then(|p| wasi_path::to_native(&p, true)) {
                if let Some(parent) = current_path.parent() {
                    dialog = dialog.set_directory(parent);
                }
                if let Some(name) = current_path.file_name().and_then(|name| name.to_str()) {
                    dialog = dialog.set_file_name(name);
                }
            }

            for (name, extensions) in filters.read().iter() {
                match extensions {
                    Extensions::Dynamic(extensions) => {
                        dialog = dialog.add_filter(name, extensions);
                    }
                    Extensions::Static(extensions) => {
                        dialog = dialog.add_filter(name, extensions);
                    }
                }
            }
            let Some(file) = dialog.pick_file().await else {
                return;
            };

            onchoose(file.path().to_path_buf());
        }
    };

    rsx! {
        div { display: "flex", align_items: "center", gap: "8px",
            {children}
            button { flex_grow: 1, onclick, "Select File" }
        }
    }
}

fn mime_desc(top: &str, sub: &str) -> String {
    let mut output = String::new();

    if sub != "*" {
        // Strip vendor and x- prefixes

        let sub = sub
            .strip_prefix("vnd.")
            .unwrap_or(sub)
            .strip_prefix("x-")
            .unwrap_or(sub);

        // Capitalize the first letter

        let mut chars = sub.chars();
        if let Some(c) = chars
            .by_ref()
            .map(|c| match c {
                '-' | '.' | '+' | '|' | '(' | ')' => ' ',
                _ => c,
            })
            .next()
        {
            output.extend(c.to_uppercase());
        }

        // Only capitalize chunks of the rest that are 4 characters or less as a
        // heuristic to detect acronyms

        let rem = chars.as_str();
        for (i, piece) in rem.split(&['-', '.', '+', '|', ' ', '(', ')']).enumerate() {
            if i != 0 {
                output.push(' ');
            }
            if piece.len() <= 4 - (i == 0) as usize {
                output.extend(piece.chars().flat_map(|c| c.to_uppercase()));
            } else {
                output.push_str(piece);
            }
        }

        output.push(' ');
    }

    let mut chars = top.chars();
    if sub == "*" {
        if let Some(c) = chars.by_ref().find(|c| *c != '(' && *c != ')') {
            for c in c.to_uppercase() {
                output.extend(c.to_uppercase());
            }
        }
    }
    output.push_str(chars.as_str());
    output.push_str(if top == "image" { "s" } else { " files" });

    output
}
