use dioxus::prelude::*;
use livesplit_auto_splitting::{AutoSplitter, CompiledAutoSplitter, Runtime, TimerState};

use crate::{GameTimeState, IdeTimer};

use super::{AutoSplitterControl, SideBar, Statistics, StatisticsData, TimerInfo};

#[component]
pub fn LeftSideBar(
    timer: SyncSignal<IdeTimer>,
    split_index: SyncSignal<usize>,
    timer_state: SyncSignal<TimerState>,
    game_time: SyncSignal<time::Duration>,
    game_time_state: SyncSignal<GameTimeState>,
    runtime: SyncSignal<Runtime>,
    module: SyncSignal<Option<CompiledAutoSplitter>>,
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    statistics: SyncSignal<StatisticsData>,
    optimize: Signal<bool>,
    width: f64,
) -> Element {
    rsx! {
        SideBar { width,
            AutoSplitterControl {
                timer,
                runtime,
                module,
                auto_splitter,
                optimize,
            }
            TimerInfo {
                split_index,
                timer_state,
                game_time,
                game_time_state,
                timer,
            }
            Statistics { statistics }
        }
    }
}
