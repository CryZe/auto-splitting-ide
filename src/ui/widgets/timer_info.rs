use std::fmt;

use dioxus::prelude::*;
use livesplit_auto_splitting::{Timer, TimerState};

use crate::{
    ui::{FmtDuration, Widget},
    GameTimeState, IdeTimer,
};

struct FmtTimerState(TimerState);

impl fmt::Display for FmtTimerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            TimerState::NotRunning => write!(f, "Not Running"),
            TimerState::Running => write!(f, "Running"),
            TimerState::Paused => write!(f, "Paused"),
            TimerState::Ended => write!(f, "Ended"),
        }
    }
}

#[component]
pub fn TimerInfo(
    split_index: SyncSignal<usize>,
    timer_state: SyncSignal<TimerState>,
    game_time: SyncSignal<time::Duration>,
    game_time_state: SyncSignal<GameTimeState>,
    timer: SyncSignal<IdeTimer>,
) -> Element {
    let is_start = *timer_state.read() == TimerState::NotRunning;
    rsx! {
        Widget { title: "Timer",
            div { class: "table",
                div { "Timer State" }
                div { "{FmtTimerState(*timer_state.read())}" }
                div { "Game Time" }
                div { "{FmtDuration(*game_time.read())}" }
                div { "Game Time State" }
                div { "{game_time_state}" }
                div { "Split Index" }
                div { "{split_index}" }
            }
            button {
                class: if is_start { "start" } else { "stop" },
                onclick: move |_| {
                    if is_start {
                        timer.write().start();
                    } else {
                        timer.write().reset();
                    }
                },
                if is_start {
                    "▶"
                } else {
                    "⏹"
                }
            }
        }
    }
}
