// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// system info panel
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    super::super::{
        super::{Core, Environment},
        ui::{Message, Tab},
    },
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{
            column, container, horizontal_rule, scrollable, text, Column, Container, Row, Space,
            Text,
        },
        Alignment, Element, Length, Renderer,
    },
    iced_aw::tab_bar::TabLabel,
    mu::{Condition, Exception, Mu, Result, System as MuSystem, Tag},
    std::sync::RwLock,
    sysinfo::{System, SystemExt},
};

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

#[derive(Debug, Clone)]
pub enum AboutMessage {
    Refresh,
}

pub struct AboutTab {
    info: System,
    console: SysCons,
}

impl AboutTab {
    pub fn new() -> Self {
        let mut info = System::new_all();
        let console = SysCons::new();

        info.refresh_all();

        Self { info, console }
    }

    pub fn log(&self, message: String) {
        self.console.log(message);
    }

    pub fn update(&mut self, message: AboutMessage) {
        match message {
            AboutMessage::Refresh => {
                let mut info = System::new_all();

                info.refresh_all();

                self.info = info;
            }
        }
    }

    fn system_info(&self, width: i32, height: i32) -> Element<AboutMessage> {
        let content: Element<_> = column![
            text("system".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
            text(format!("host name: {}", self.info.host_name().unwrap())).size(20),
            text(format!("system name: {}", self.info.name().unwrap())).size(20),
            text(format!(
                "kernel version: {}",
                self.info.kernel_version().unwrap()
            ))
            .size(20),
            text(format!("OS version: {}", self.info.os_version().unwrap())).size(20),
        ]
        .width(width as f32)
        .height(height as f32)
        .spacing(2)
        .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn mu_info(&self, env: &Environment, width: i32, height: i32) -> Element<AboutMessage> {
        let content: Element<_> = column![
            text("mu".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
            text(format!("mu: version: {}", Mu::VERSION)).size(20),
            text(format!(
                "core: heap size (pages) : {}",
                env.core
                    .as_ref()
                    .unwrap()
                    .eval("(mu:sv-ref (mu:hp-info) 1)".to_string())
                    .1
            ))
            .size(20),
        ]
        .width(width as f32)
        .height(height as f32)
        .spacing(2)
        .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn view(&self, env: &Environment) -> Element<'_, Message, Renderer> {
        let content: Element<'_, AboutMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(800)
                .padding(20)
                .push(
                    Row::new()
                        .align_items(Alignment::Start)
                        .height(175)
                        .padding(20)
                        .push(self.system_info(350, 175))
                        .push(self.mu_info(env, 350, 175)),
                )
                .push(self.console(800, 150)),
        )
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::About)
    }

    fn console(&self, width: i32, height: i32) -> Element<AboutMessage> {
        let content = column![
            text("console log"),
            horizontal_rule(1),
            text(self.console.contents().unwrap()).size(15)
        ]
        .padding(20)
        .width(width as f32)
        .height(height as f32)
        .align_items(Alignment::Start)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Tab for AboutTab {
    type Message = Message;

    fn title(&self) -> String {
        // String::from("Eth::System")
        String::new()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("about".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
