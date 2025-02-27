use std::fs::read_dir;

use egui::Vec2;
use serde::{Deserialize, Serialize};

use crate::{get_config_path, run::game::SmallGame};

use super::columns::ColumnVec;

#[derive(Deserialize)]
pub struct OptionalConfig {
    dark_mode: Option<bool>,
    next_split_as_reset: Option<bool>,
    autosave: Option<bool>,
    autosave_interval: Option<u64>,
    game_list: Option<Vec<SmallGame>>,
    font_size: Option<f32>,
    limit_splits_shown: Option<bool>,
    num_splits_counter: Option<usize>,
    counter_size: Option<Vec2>,
    columns: Option<ColumnVec>,
}

impl OptionalConfig {
    fn to_config(&self) -> Config {
        Config {
            dark_mode: self.dark_mode.unwrap_or(true),
            next_split_as_reset: self.next_split_as_reset.unwrap_or(true),
            autosave: self.autosave.unwrap_or(true),
            autosave_interval: self.autosave_interval.unwrap_or(60),
            game_list: match &self.game_list {
                None => Vec::new(),
                Some(v) => v.to_vec(),
            },
            font_size: self.font_size.unwrap_or(14.0),
            limit_splits_shown: self.limit_splits_shown.unwrap_or(false),
            num_splits_counter: self.num_splits_counter.unwrap_or(10),
            counter_size: self.counter_size.unwrap_or([280.0, 600.0].into()),
            columns: self.columns.clone().unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Config {
    pub dark_mode: bool,
    pub next_split_as_reset: bool,
    pub autosave: bool,
    pub autosave_interval: u64,
    pub game_list: Vec<SmallGame>,
    pub font_size: f32,
    pub limit_splits_shown: bool,
    pub num_splits_counter: usize,
    pub counter_size: Vec2,
    pub columns: ColumnVec,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dark_mode: true,
            next_split_as_reset: true,
            autosave: true,
            autosave_interval: 60,
            game_list: Vec::new(),
            font_size: 14.0,
            limit_splits_shown: false,
            num_splits_counter: 0,
            counter_size: [280.0, 600.0].into(),
            columns: ColumnVec::default(),
        }
    }
}

impl Config {
    pub fn save(&mut self) {
        let config_path = get_config_path();
        let config_str = serde_json::to_string(self).unwrap();
        let _ = std::fs::write(format!("{config_path}/config.json"), config_str);
    }

    pub fn load() -> Self {
        let config_path = get_config_path();

        if read_dir(&config_path).is_err() {
            let _ = std::fs::create_dir(&config_path);
        }

        if read_dir(format!("{config_path}/games")).is_err() {
            let _ = std::fs::create_dir(format!("{config_path}/games"));
        }

        if read_dir(format!("{config_path}/categories")).is_err() {
            let _ = std::fs::create_dir(format!("{config_path}/categories"));
        }

        let config_json: String =
            match std::fs::read_to_string(format!("{config_path}/config.json")) {
                Err(_) => {
                    let tmp: Config = Default::default();
                    let config_str = serde_json::to_string(&tmp).unwrap();
                    let _ =
                        std::fs::write(format!("{config_path}/config.json"), config_str.clone());
                    config_str
                }
                Ok(f) => f,
            };

        serde_json::from_str::<OptionalConfig>(config_json.as_str())
            .unwrap()
            .to_config()
    }
}
