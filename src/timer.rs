use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use dioxus::prelude::*;
use indexmap::IndexMap;
use livesplit_auto_splitting::{
    settings, AutoSplitter, CompiledAutoSplitter, Config, LogLevel, Runtime, Timer, TimerState,
};

use crate::{LogEntries, StatisticsData, Widgets};

#[derive(PartialEq)]
pub enum GameTimeState {
    NotInitialized,
    Running,
    Paused,
}

impl fmt::Display for GameTimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameTimeState::NotInitialized => write!(f, "Not Initialized"),
            GameTimeState::Running => write!(f, "Running"),
            GameTimeState::Paused => write!(f, "Paused"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct IdeTimer {
    pub split_index: SyncSignal<usize>,
    pub segment_splitted: SyncSignal<Vec<bool>>,
    pub timer_state: SyncSignal<TimerState>,
    pub game_time: SyncSignal<time::Duration>,
    pub game_time_state: SyncSignal<GameTimeState>,
    pub variables: SyncSignal<IndexMap<String, String>>,
    pub processes: SyncSignal<Vec<(String, String)>>,
    pub settings_widgets: SyncSignal<Widgets>,
    pub settings_map: SyncSignal<settings::Map>,
    pub logs: SyncSignal<LogEntries>,
    pub wasm_path: SyncSignal<Option<PathBuf>>,
    pub statistics: SyncSignal<StatisticsData>,
}

enum Load<'a> {
    File(&'a Path),
    Reload,
    Restart,
}

impl IdeTimer {
    pub fn load_file(
        &self,
        file: &Path,
        runtime: SyncSignal<Runtime>,
        module: SyncSignal<Option<CompiledAutoSplitter>>,
        auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    ) {
        self.load(Load::File(file), runtime, module, auto_splitter);
    }

    pub fn reload(
        &self,
        runtime: SyncSignal<Runtime>,
        module: SyncSignal<Option<CompiledAutoSplitter>>,
        auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    ) {
        self.load(Load::Reload, runtime, module, auto_splitter);
    }

    pub fn restart(
        &self,
        runtime: SyncSignal<Runtime>,
        module: SyncSignal<Option<CompiledAutoSplitter>>,
        auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    ) {
        self.load(Load::Restart, runtime, module, auto_splitter);
    }

    fn load(
        &self,
        load: Load<'_>,
        runtime: SyncSignal<Runtime>,
        mut module: SyncSignal<Option<CompiledAutoSplitter>>,
        mut auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
    ) {
        let settings_map = if let Load::File(path) = load {
            { self.wasm_path }.set(Some(path.to_path_buf()));
            None
        } else {
            auto_splitter.read().as_ref().map(|r| r.settings_map())
        };

        let mut succeeded = true;

        if let (Load::File(_) | Load::Reload, Some(path)) = (&load, &*self.wasm_path.read()) {
            module.set(
                match fs::read(path)
                    .context("Failed loading the auto splitter from the file system.")
                    .and_then(|data| {
                        runtime
                            .read()
                            .compile(&data)
                            .context("Failed loading the auto splitter.")
                    }) {
                    Ok(module) => Some(module),
                    Err(e) => {
                        succeeded = false;
                        { self.logs }
                            .write()
                            .push_level(format!("{e:?}"), LogLevel::Error);
                        None
                    }
                },
            );

            // self.module_modified_time = fs::metadata(path).ok().and_then(|m| m.modified().ok());
        }

        let new_auto_splitter = if let Some(module) = &*module.read() {
            match module
                .instantiate(*self, settings_map, None)
                .context("Failed starting the auto splitter.")
            {
                Ok(r) => Some(r),
                Err(e) => {
                    succeeded = false;
                    { self.logs }
                        .write()
                        .push_level(format!("{e:?}"), LogLevel::Error);
                    None
                }
            }
        } else {
            None
        };

        // self.kill_auto_splitter_if_it_doesnt_react();
        auto_splitter.set(new_auto_splitter);

        // *self.slowest_tick.lock().unwrap() = std::time::Duration::ZERO;
        // self.avg_tick_secs.store(0.0, atomic::Ordering::Relaxed);
        // self.tick_times.lock().unwrap().clear();

        // let mut timer = self.timer.0.write().unwrap();
        // if let Load::File(_) = &load {
        //     timer.clear();
        // }
        // timer.variables.clear();

        if succeeded {
            { self.logs }.write().push_level(
                match load {
                    Load::File(_) => "Auto splitter loaded.",
                    Load::Reload => "Auto splitter reloaded.",
                    Load::Restart => "Auto splitter restarted.",
                }
                .into(),
                LogLevel::Info,
            );
        }
    }

    fn timer_state(&self) -> TimerState {
        *self.timer_state.read()
    }
}

impl Timer for IdeTimer {
    fn state(&self) -> TimerState {
        self.timer_state()
    }

    fn start(&mut self) {
        if self.timer_state() == TimerState::NotRunning {
            self.timer_state.set(TimerState::Running);
            self.logs
                .write()
                .push_level("Timer started.".to_string(), LogLevel::Debug);
        }
    }

    fn split(&mut self) {
        if self.timer_state() == TimerState::Running {
            self.segment_splitted.write().push(true);
            *self.split_index.write() += 1;
            self.logs
                .write()
                .push_level("Splitted.".to_string(), LogLevel::Debug);
        }
    }

    fn skip_split(&mut self) {
        if self.timer_state() == TimerState::Running {
            self.segment_splitted.write().push(false);
            *self.split_index.write() += 1;
            self.logs
                .write()
                .push_level("Split skipped.".to_string(), LogLevel::Debug);
        }
    }

    fn undo_split(&mut self) {
        if self.timer_state() == TimerState::Ended {
            self.timer_state.set(TimerState::Running);
        }
        if self.timer_state() == TimerState::Running {
            self.segment_splitted.write().pop();
            let new_split = self.split_index.read().saturating_sub(1);
            *self.split_index.write() = new_split;
            self.logs
                .write()
                .push_level("Split undone.".to_string(), LogLevel::Debug);
        }
    }

    fn reset(&mut self) {
        if self.timer_state() != TimerState::NotRunning {
            self.timer_state.set(TimerState::NotRunning);
            self.split_index.set(0);
            self.segment_splitted.write().clear();
            self.game_time.set(time::Duration::ZERO);
            self.game_time_state.set(GameTimeState::NotInitialized);
            self.variables.write().clear();
            self.logs
                .write()
                .push_level("Timer reset.".to_string(), LogLevel::Debug);
        }
    }

    fn set_game_time(&mut self, time: time::Duration) {
        self.game_time.set(time);
        if *self.game_time_state.read() != GameTimeState::Running {
            self.game_time_state.set(GameTimeState::Running);
        }
    }

    fn pause_game_time(&mut self) {
        if *self.game_time_state.read() != GameTimeState::Paused {
            self.game_time_state.set(GameTimeState::Paused);
        }
    }

    fn resume_game_time(&mut self) {
        if *self.game_time_state.read() != GameTimeState::Running {
            self.game_time_state.set(GameTimeState::Running);
        }
    }

    fn set_variable(&mut self, key: &str, value: &str) {
        if self.variables.read().get(key).is_none_or(|v| v != value) {
            self.variables
                .write()
                .insert(key.to_string(), value.to_string());
        }
    }

    fn log_auto_splitter(&mut self, message: fmt::Arguments<'_>) {
        self.logs.write().push(format!("{message}"));
    }

    fn log_runtime(&mut self, message: fmt::Arguments<'_>, log_level: LogLevel) {
        self.logs
            .write()
            .push_level(format!("{message}"), log_level);
    }

    fn current_split_index(&self) -> Option<usize> {
        Some(*self.split_index.read())
    }

    fn segment_splitted(&self, index: usize) -> Option<bool> {
        self.segment_splitted.read().get(index).copied()
    }
}

pub fn build_runtime(optimize: bool) -> Runtime {
    let mut config = Config::default();
    config.debug_info = true;
    config.optimize = optimize;
    Runtime::new(config).unwrap()
}
