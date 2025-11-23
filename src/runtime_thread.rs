use std::{
    sync::{atomic::AtomicBool, Arc},
    thread,
    time::Instant,
};

use dioxus::prelude::*;
use livesplit_auto_splitting::{AutoSplitter, LogLevel, Timer};

use crate::IdeTimer;

pub static RUNNING: AtomicBool = AtomicBool::new(true);

pub fn run(
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    mut timer: SyncSignal<IdeTimer>,
) {
    let mut next_tick = Instant::now();
    while RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        let tick_rate = if let Some(auto_splitter) = auto_splitter.read().as_ref() {
            let mut auto_splitter_lock = auto_splitter.lock();
            let now = Instant::now();
            let res = auto_splitter_lock.update();
            let time_of_tick = now.elapsed();
            let memory_usage = auto_splitter_lock.memory().len();
            {
                // TODO: Improve perf
                let mut processes = timer.read().processes;
                let mut processes = processes.write();
                processes.clear();
                auto_splitter_lock.attached_processes().for_each(|process| {
                    processes.push((
                        process.pid().to_string(),
                        process.path().unwrap_or("Unnamed Process").to_owned(),
                    ));
                });
            }
            let handles = auto_splitter_lock.handles();
            drop(auto_splitter_lock);

            {
                let mut stats = timer.read().statistics;
                let stats = &mut *stats.write();
                stats.memory_usage = memory_usage;
                stats.handles = handles;
                if time_of_tick > stats.slowest_tick {
                    stats.slowest_tick = time_of_tick;
                }
                stats.tick_rate = auto_splitter.tick_rate();
                stats.avg_tick_secs =
                    stats.avg_tick_secs * 0.999 + time_of_tick.as_secs_f64() * 0.001;
            }

            if let Err(e) = res {
                timer.write().log_runtime(
                    format_args!("{:?}", e.context("Failed executing the auto splitter.")),
                    LogLevel::Error,
                )
            };

            let mut settings_widgets = timer.read().settings_widgets;
            let new_widgets = auto_splitter.settings_widgets();
            if !Arc::ptr_eq(&settings_widgets.read().0, &new_widgets) {
                settings_widgets.write().0 = new_widgets;
            }

            let mut settings_map = timer.read().settings_map;
            let new_settings_map = auto_splitter.settings_map();
            if !settings_map.read().is_unchanged(&new_settings_map) {
                settings_map.set(new_settings_map);
            }

            auto_splitter.tick_rate()
        } else {
            let mut processes = timer.read().processes;
            let mut processes = processes.write();
            processes.clear();

            // Tick at 10 Hz when no runtime is loaded.
            std::time::Duration::from_secs(1) / 10
        };

        next_tick += tick_rate;

        let now = Instant::now();
        if let Some(sleep_time) = next_tick.checked_duration_since(now) {
            thread::sleep(sleep_time);
        } else {
            // In this case we missed the next tick already. This likely comes
            // up when the operating system was suspended for a while. Instead
            // of trying to catch up, we just reset the next tick to start from
            // now.
            next_tick = now;
        }
    }
}
