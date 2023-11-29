//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(dead_code)]
mod config;
mod core;
mod eth;

use {
    crate::{config::Config, core::Core},
    eth::ui::Ui,
    iced::{window, Application, Settings},
};

#[derive(Default)]
pub struct Environment {
    config: (Option<bool>, config::Config),
    config_path: std::path::PathBuf,
    home_path: std::path::PathBuf,
    hostname: String,
    user: String,
    core: Option<Core>,
}

impl Environment {
    const CONFIG_PATH: &'static str = ".config/eth";
    const CONFIG_FILE: &'static str = "config.json";

    fn dotfiles(self) -> Self {
        let config_path = self.config_path.as_path();

        if !config_path.exists() {
            panic!(
                "config directory {:?} does not exist, not correctly installed",
                config_path
            )
        }

        let config = config::Config::from_env(&self);
        let core = Some(Core::new(&config, &self.config_path));

        Environment {
            user: self.user,
            hostname: self.hostname,
            home_path: self.home_path,
            config_path: self.config_path,
            config,
            core,
        }
    }
}

pub fn main() -> iced::Result {
    let home = &envmnt::get_or("HOME", "");
    let home_path = std::path::Path::new(home);
    let config_path = std::path::Path::new(Environment::CONFIG_PATH);

    let env = Environment {
        user: whoami::username(),
        hostname: whoami::hostname(),
        home_path: home_path.to_path_buf(),
        config_path: std::path::Path::join(home_path, config_path),
        core: None,
        config: (None, Config::default()),
    }
    .dotfiles();

    let settings = Settings {
        exit_on_close_request: true,
        flags: env,
        window: window::Settings {
            size: (1000, 500),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        // default_font: Some(include_bytes!("path-to-font ttf")),
        antialiasing: true,
        ..Default::default()
    };

    Ui::run(settings)
}
