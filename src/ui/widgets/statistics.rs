use std::time::Duration;

use dioxus::prelude::*;

use crate::ui::FmtDuration;

use super::Widget;

#[derive(Default)]
pub struct StatisticsData {
    pub tick_rate: Duration,
    pub avg_tick_secs: f64,
    pub slowest_tick: Duration,
    pub handles: u64,
    pub memory_usage: usize,
}

#[component]
pub fn Statistics(statistics: SyncSignal<StatisticsData>) -> Element {
    let statistics = &*statistics.read();
    rsx! {
        Widget { title: "Statistics",
            div { class: "table",
                div { "Tick Rate" }
                div { "{FmtDuration(statistics.tick_rate)}" }
                div { "Avg. Tick Rate" }
                div { "{FmtDuration(Duration::from_secs_f64(statistics.avg_tick_secs))}" }
                div { "Slowest Tick" }
                div { "{FmtDuration(statistics.slowest_tick)}" }
                div { "Handles" }
                div { "{statistics.handles}" }
                div { "Memory" }
                div {
                    "{byte_unit::Byte::from_u64(statistics.memory_usage as _)
                        .get_appropriate_unit(byte_unit::UnitType::Binary)}"
                }
            }
        }
    }
}
