// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// namespaced symbols panel
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    super::{
        super::super::{Core, Environment},
        super::ui::{Message, Tab},
    },
    iced::widget::scrollable,
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{column, container, horizontal_rule, text, Column, Container, Row, Space, Text},
        Alignment, Element, Length, Renderer,
    },
    iced_aw::tab_bar::TabLabel,
    mu::{Condition, Exception, Mu, Result, System as MuSystem, Tag},
    sysinfo::{System, SystemExt},
};

#[derive(Debug, Clone)]
pub enum NamespacesMessage {
    Refresh,
}

pub struct NamespacesTab {
    names: Vec<String>,
}

impl NamespacesTab {
    pub fn new() -> Self {
        let names = vec!["nil".to_string(), "mu".to_string(), "core".to_string()];

        // names.refresh_all();

        Self { names }
    }

    pub fn update(&mut self, message: NamespacesMessage) {
        match message {
            NamespacesMessage::Refresh => {
                let names = vec!["nil".to_string(), "mu".to_string(), "core".to_string()];

                // names.refresh_all();

                self.names = names;
            }
        }
    }

    fn namespace_info(&self, width: i32, height: i32) -> Element<NamespacesMessage> {
        let column = column!(
            text("namespaces".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
        );

        let mut ns_col = column;

        for ns in self.names.iter() {
            ns_col = ns_col.push(text(ns.to_string()).size(20));
        }

        let content: Element<_> = ns_col
            .width(width as f32)
            .height(height as f32)
            .spacing(2)
            .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn namespace_symbols(
        &self,
        _: &Environment,
        width: i32,
        height: i32,
    ) -> Element<NamespacesMessage> {
        let column = column!(
            text("symbols".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
        );

        let mut ns_col = column;

        for ns in self.names.iter() {
            ns_col = ns_col.push(text(ns.to_string()).size(20));
        }

        let content: Element<_> = ns_col
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
        let content: Element<'_, NamespacesMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(800)
                .padding(20)
                .push(
                    Row::new()
                        .align_items(Alignment::Start)
                        .height(175)
                        .padding(20)
                        .push(self.namespace_info(350, 175))
                        .push(self.namespace_symbols(env, 350, 175)),
                ),
        )
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::Namespaces)
    }
}

impl Tab for NamespacesTab {
    type Message = Message;

    fn title(&self) -> String {
        String::new()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("namespaces".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
