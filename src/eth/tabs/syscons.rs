//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// system console panel
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use std::sync::RwLock;

pub struct SysCons {
    text: RwLock<Vec<String>>,
}

impl SysCons {
    pub fn new() -> Self {
        SysCons {
            text: RwLock::new(Vec::<String>::new()),
        }
    }

    pub fn log(&self, message: String) {
        let mut text = self.text.write().unwrap();
        let now = chrono::Utc::now();
        let now_str = now.format("%m%d%H%M%S");

        text.push(format!("{}: {}", now_str, message));
    }

    pub fn contents(&self) -> Option<String> {
        let text = self.text.read().unwrap();

        if text.len() == 0 {
            None
        } else {
            Some(text.join("\n"))
        }
    }
}
