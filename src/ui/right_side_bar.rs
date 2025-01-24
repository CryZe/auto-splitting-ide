use dioxus::prelude::*;
use indexmap::IndexMap;
use livesplit_auto_splitting::{settings, AutoSplitter};

use crate::{IdeTimer, Widgets};

use super::{Processes, SettingsGui, SettingsMap, SideBar, Variables};

#[component]
pub fn RightSideBar(
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    variables: SyncSignal<IndexMap<String, String>>,
    processes: SyncSignal<Vec<(String, String)>>,
    settings_widgets: SyncSignal<Widgets>,
    settings_map: SyncSignal<settings::Map>,
    width: f64,
) -> Element {
    rsx! {
        SideBar { drag_side: false, width,
            Variables { variables }
            Processes { processes }
            SettingsGui { settings_widgets, settings_map, auto_splitter }
            SettingsMap { settings_map, auto_splitter }
        }
    }
}
