// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// namespaced symbols panel
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]
use {
    super::super::{
        super::{Core, Environment},
        ui::{Message, Tab},
    },
    iced::widget::scrollable,
    iced::{
        alignment::{Horizontal, Vertical},
        widget::{column, container, horizontal_rule, text, Column, Container, Row, Space, Text},
        Alignment, Element, Font, Length, Renderer,
    },
    iced_aw::{
        selection_list::{SelectionList, SelectionListStyles},
        tab_bar::TabLabel,
    },
    mu::{Condition, Exception, Mu, Result, System as MuSystem, Tag},
    sysinfo::{System, SystemExt},
};

#[derive(Debug, Clone)]
pub enum SymbolsMessage {
    Selected(String),
    Refresh,
}

pub struct SymbolsTab {
    namespaces: Vec<String>,
    symbols: Vec<String>,
}

impl SymbolsTab {
    pub fn new() -> Self {
        let namespaces = vec!["nil".to_string(), "mu".to_string(), "core".to_string()];

        Self {
            namespaces,
            symbols: vec![],
        }
    }

    pub fn update(&mut self, message: SymbolsMessage) {
        match message {
            SymbolsMessage::Selected(str) => {
                println!("selects: {}", str)
            }
            SymbolsMessage::Refresh => {
                let names = vec!["nil".to_string(), "mu".to_string(), "core".to_string()];

                // names.refresh_all();

                self.namespaces = names;
            }
        }
    }

    fn namespaces(&self, width: i32, height: i32) -> Element<SymbolsMessage> {
        let selection_list = SelectionList::new_with(
            &self.namespaces,
            SymbolsMessage::Selected,
            12.0,
            5.0,
            SelectionListStyles::Default,
        )
        .width(Length::Shrink)
        .height(Length::Fixed(100.0));

        let column = column!(
            text("namespaces:".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
            selection_list,
        );

        let content: Element<_> = column
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
    ) -> Element<SymbolsMessage> {
        let column = column!(
            text("namespaces:".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
        );

        let ns_col = column;

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

    fn symbol(&self, _: &Environment, width: i32, height: i32) -> Element<SymbolsMessage> {
        let column = column!(
            text("symbol:".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
        );

        let ns_col = column;

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

    fn inspect(&self, _: &Environment, width: i32, height: i32) -> Element<SymbolsMessage> {
        let column = column!(
            text("inspect:".to_string()).size(20),
            horizontal_rule(1),
            Space::new(width as u16, 5),
        );

        let ns_col = column;

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
        let content: Element<'_, SymbolsMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(800)
                .padding(20)
                .push(
                    Row::new()
                        .align_items(Alignment::Start)
                        .height(175)
                        .padding(20)
                        .push(self.namespaces(200, 175))
                        .push(self.namespace_symbols(env, 200, 175))
                        .push(self.symbol(env, 200, 175))
                        .push(self.inspect(env, 200, 175)),
                ),
        )
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::Symbols)
    }
}

impl Tab for SymbolsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::new()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("symbols".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
