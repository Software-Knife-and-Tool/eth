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
    iced::{
        alignment::{Horizontal, Vertical},
        theme,
        widget::{button, column, row, scrollable},
        widget::{container, horizontal_rule, text},
        widget::{Column, Container, Row, Space, Text},
        Alignment, Background, Color, Element, Font, Length, Renderer,
    },
    iced_aw::{
        selection_list::{selection_list, SelectionList, SelectionListStyles},
        tab_bar::TabLabel,
    },
    mu::{Condition, Exception, Mu, Result, System as MuSystem, Tag},
    sysinfo::{System, SystemExt},
};

// inspector
#[derive(Debug, Clone)]
pub enum InspectorMessage {
    SelectSymbol(String),
    SelectNamespace(String),
    Clear,
    Refresh,
}

pub struct InspectorTab {
    namespaces: Option<Vec<String>>,
    selected_ns: Option<String>,
    inspector: Option<Vec<String>>,
    selected_symbol: Option<String>,
    describe: Option<String>,
}

impl InspectorTab {
    pub fn new() -> Self {
        Self {
            namespaces: None,
            selected_ns: None,
            inspector: None,
            selected_symbol: None,
            describe: None,
        }
    }

    fn trimr(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next_back();
        chars.as_str()
    }

    fn triml(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.as_str()
    }

    fn fetch_ns_list(env: &Environment) -> Vec<String> {
        let ns_list = Core::eval_rstring(
            &env.core.as_ref().unwrap().system,
            "(mu:ns-map)".to_string(),
        );
        let ns_string = env.core.as_ref().unwrap().system.write(ns_list, false);
        let trim = Self::triml(Self::trimr(&ns_string)).to_string();
        let slice = &trim.split(' ').collect::<Vec<&str>>();

        let mut ns = vec![];

        for ns_ in slice {
            ns.push(ns_.to_string())
        }

        ns
    }

    fn fetch_symbols_list(env: &Environment, ns: String) -> Vec<String> {
        let sym_list = Core::eval_rstring(
            &env.core.as_ref().unwrap().system,
            format!("(mu:ns-syms :list {})", ns).to_string(),
        );
        let sym_str = env.core.as_ref().unwrap().system.write(sym_list, false);
        let trim = Self::triml(Self::trimr(&sym_str)).to_string();
        let slice = &trim.split(' ').collect::<Vec<&str>>();

        let mut syms = vec![];

        for sym in slice {
            syms.push(sym.to_string())
        }

        syms
    }

    fn describe_symbol(env: &Environment, ns: &str, symbol: &String) -> String {
        let describe_cmd = format!("(eth:describe '{}:{})", Self::triml(ns), symbol).to_string();
        let descr_str = Core::eval_rstring(&env.core.as_ref().unwrap().system, describe_cmd);

        env.core.as_ref().unwrap().system.write(descr_str, false)
    }

    pub fn update(&mut self, env: &Environment, message: InspectorMessage) {
        match message {
            InspectorMessage::SelectSymbol(str) => {
                self.selected_symbol = Some(str);
                self.describe = Some(Self::describe_symbol(
                    env,
                    self.selected_ns.as_ref().unwrap(),
                    self.selected_symbol.as_ref().unwrap(),
                ));
            }
            InspectorMessage::SelectNamespace(str) => {
                self.selected_ns = Some(str.clone());
                self.selected_symbol = None;
                self.describe = None;
                self.inspector = Some(Self::fetch_symbols_list(env, str.to_string()));
            }
            InspectorMessage::Refresh => match self.namespaces {
                Some(_) => (),
                None => self.namespaces = Some(Self::fetch_ns_list(env)),
            },
            InspectorMessage::Clear => {
                self.selected_ns = None;
                self.selected_symbol = None;
                self.inspector = None;
                self.describe = None
            }
        }
    }

    fn namespaces(&self, width: i32, height: i32) -> Element<InspectorMessage> {
        let column = match &self.namespaces {
            Some(symvec) => {
                let selection_list = SelectionList::new_with(
                    symvec,
                    InspectorMessage::SelectNamespace,
                    16.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Shrink)
                .height(Length::Fixed(100.0));

                column!(
                    text("namespaces:".to_string()).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                    selection_list,
                )
            }
            None => {
                column!(
                    text("namespaces:".to_string()).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                )
            }
        };

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
    ) -> Element<InspectorMessage> {
        let column = match &self.inspector {
            Some(symvec) => {
                let selection_list = SelectionList::new_with(
                    symvec,
                    InspectorMessage::SelectSymbol,
                    16.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Shrink)
                .height(Length::Fixed(100.0));

                column!(
                    text(self.selected_ns.as_ref().unwrap()).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                    selection_list,
                )
            }
            None => {
                column!(
                    text(" ".to_string()).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                )
            }
        };

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

    fn symbol(&self, _: &Environment, width: i32, height: i32) -> Element<InspectorMessage> {
        fn rem_first(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.as_str()
        }

        match &self.describe {
            Some(descr) => {
                let column = column!(
                    text(format!(
                        "{}:{}",
                        rem_first(self.selected_ns.as_ref().unwrap()),
                        self.selected_symbol.as_ref().unwrap()
                    ))
                    .size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                    text(descr).size(16),
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
            None => {
                let blank = " ".to_string();

                let column = column!(
                    text(&blank).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
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
        }
    }

    fn inspect(&self, _: &Environment, width: i32, height: i32) -> Element<InspectorMessage> {
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
        let controls = row![
            button(text("clear".to_string()).size(13))
                .height(28)
                .style(theme::Button::Primary)
                .on_press(InspectorMessage::Clear),
            button(text("refresh".to_string()).size(13))
                .height(28)
                .style(theme::Button::Primary)
                .on_press(InspectorMessage::Refresh)
        ]
        .spacing(8);

        let content: Element<'_, InspectorMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(800)
                .padding(20)
                .push(controls.width(Length::Fill))
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

        content.map(Message::Inspector)
    }
}

impl Tab for InspectorTab {
    type Message = Message;

    fn title(&self) -> String {
        String::new()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("inspector".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
