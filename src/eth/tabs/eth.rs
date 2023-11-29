//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use {
    crate::Environment,
    serde::{Deserialize, Serialize},
    serde_json::{Result as SerdeResult, Value},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CoreButton {
    pub group: String,
    pub label: String,
    pub form: String,
}

impl CoreButton {
    pub fn new() -> Self {
        Self {
            group: String::new(),
            label: String::new(),
            form: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Eth {
    Defbutton {
        group: String,
        label: String,
        on_press: String,
    },
}

impl Eth {
    pub fn run(env: &Environment) {
        let cmd_stream = env.core.as_ref().unwrap().cmd_stream;
        let eth_json = env
            .core
            .as_ref()
            .unwrap()
            .system
            .mu()
            .get_string(cmd_stream)
            .unwrap();

        if !eth_json.is_empty() {
            let eth: Eth = serde_json::from_str(&eth_json).unwrap();

            println!("command: {}", eth_json);
            match eth {
                Eth::Defbutton {
                    group: _,
                    label: _,
                    on_press: _,
                } => {}
            }
        }
    }

    pub fn on_press(_button: &CoreButton, _env: &Environment) {}
}
