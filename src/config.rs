//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    crate::Environment,
    serde::{Deserialize, Serialize},
    std::{fs::File, io::BufReader},
};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub window: Option<Option<Window>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub textui: Option<Option<TextUi>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub mu: Option<Option<String>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Window {
    #[serde(default, with = "::serde_with::rust::double_option")]
    size: Option<Option<(u32, u32)>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    min_size: Option<Option<(u32, u32)>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    max_size: Option<Option<(u32, u32)>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    resizable: Option<Option<bool>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct TextUi {
    #[serde(default, with = "::serde_with::rust::double_option")]
    rows: Option<Option<usize>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    cursor: Option<Option<usize>>,
}

impl Window {
    pub fn size(config: &Config) -> (u32, u32) {
        match &config.window {
            Some(None) | None => (1024, 768),
            Some(Some(window)) => match window.size {
                Some(None) | None => (1024, 768),
                Some(tuple) => tuple.unwrap(),
            },
        }
    }

    pub fn min_size(config: &Config) -> (u32, u32) {
        match &config.window {
            Some(None) | None => (1024, 768),
            Some(Some(window)) => match window.min_size {
                Some(None) | None => (1024, 768),
                Some(tuple) => tuple.unwrap(),
            },
        }
    }

    pub fn max_size(config: &Config) -> (u32, u32) {
        match &config.window {
            Some(None) | None => (1024, 768),
            Some(Some(window)) => match window.max_size {
                Some(None) | None => (1024, 768),
                Some(tuple) => tuple.unwrap(),
            },
        }
    }

    pub fn resizeable(config: &Config) -> bool {
        match &config.window {
            Some(None) | None => false,
            Some(Some(window)) => match window.resizable {
                Some(None) | None => false,
                Some(resizeable) => resizeable.unwrap(),
            },
        }
    }
}

impl TextUi {
    pub fn rows(config: &Config) -> usize {
        match &config.textui {
            Some(None) | None => 25,
            Some(Some(window)) => match window.rows {
                Some(None) | None => 25,
                Some(rows) => rows.unwrap(),
            },
        }
    }

    pub fn cursor(config: &Config) -> usize {
        match &config.textui {
            Some(None) | None => 0,
            Some(Some(window)) => match window.cursor {
                Some(None) | None => 0,
                Some(cursor) => cursor.unwrap(),
            },
        }
    }
}

impl Config {
    const DEFAULT: Config = Config {
        window: Option::None,
        textui: Option::None,
        mu: Option::None,
    };

    pub fn mu(&self) -> String {
        match &self.mu {
            Some(None) | None => "".to_string(),
            Some(Some(string)) => string.to_string(),
        }
    }

    pub fn from_env(env: &Environment) -> (Option<bool>, Self) {
        let dot_path = env.config_path.as_path();

        if dot_path.exists() {
            let config = std::path::Path::new(Environment::CONFIG_FILE);
            let conf_path = std::path::Path::join(dot_path, config);

            match File::open(conf_path) {
                Err(_) => (None, Config::default()),
                Ok(file) => {
                    let reader = BufReader::new(file);

                    match serde_json::from_reader(reader) {
                        Err(_) => (None, Config::default()),
                        Ok(conf) => (Some(false), conf),
                    }
                }
            }
        } else {
            (None, Config::default())
        }
    }
}
