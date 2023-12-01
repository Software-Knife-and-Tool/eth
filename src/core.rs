//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT
#![allow(clippy::collapsible_match)]
#![allow(unused_imports)]

use {
    crate::{Config, Environment},
    mu::{Condition, Exception, Mu, Result, System, Tag},
    serde::{Deserialize, Serialize},
    std::path::Path,
};

pub struct Core {
    pub system: System,
    pub nil: Tag,
    pub init_loaded: bool,
    pub eval_stream: Tag,
    pub cmd_stream: Tag,
}

impl Core {
    pub fn new(config: &(Option<bool>, Config), config_path: &std::path::Path) -> Self {
        let (_, conf) = config;

        let system = System::new(&System::config(&conf.mu()).unwrap());

        let init_path = std::path::Path::join(config_path, "init.l");

        if !init_path.exists() {
            panic!("config: init.l not found, not installed correctly");
        }

        let init_loaded = system
            .load(&init_path.to_str().unwrap().to_string())
            .is_ok();

        let nil = Self::eval_rstring(&system, "()".to_string());
        let (cmd_stream, eval_stream) = if init_loaded {
            (
                if init_loaded {
                    Self::eval_rstring(&system, "eth:json-cmd-stream".to_string())
                } else {
                    nil
                },
                Self::eval_rstring(&system, "(mu:open :string :output \"\")".to_string()),
            )
        } else {
            (nil, system.mu().std_out())
        };

        Self {
            system,
            nil,
            eval_stream,
            init_loaded,
            cmd_stream,
        }
    }

    fn null(&self, tag: Tag) -> bool {
        self.system.mu().eq(self.nil, tag)
    }

    fn eval_rstring(system: &System, expr: String) -> Tag {
        system
            .mu()
            .eval(
                system
                    .mu()
                    .compile(system.mu().read_string(expr).unwrap())
                    .unwrap(),
            )
            .unwrap()
    }

    pub fn eval(&self, expr: String) -> (Tag, String) {
        let value = self
            .system
            .mu()
            .eval(
                self.system
                    .mu()
                    .compile(self.system.mu().read_string(expr).unwrap())
                    .unwrap(),
            )
            .unwrap();

        self.system
            .mu()
            .write(value, false, self.eval_stream)
            .unwrap();
        (
            value,
            self.system.mu().get_string(self.eval_stream).unwrap(),
        )
    }
}
