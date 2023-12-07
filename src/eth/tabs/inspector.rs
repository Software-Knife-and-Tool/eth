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
pub struct InspectorTab {
    namespaces: Option<Vec<String>>,
    namespace: Option<String>,
    namespace_symbols: Option<Vec<String>>,
    symbol: Option<String>,
    symbol_tag_keys: Option<Vec<String>>,
    symbol_tag_key: Option<String>,
    symbol_tag_values: Option<Vec<String>>,
    symbol_tag_value: Option<String>,
    object_info: Option<String>,
}

#[derive(Debug, Clone)]
pub enum InspectorMessage {
    SelectSymbol(String),
    SelectNamespace(String),
    SelectSymbolTagKey(String),
    Clear,
    Refresh,
}

impl InspectorTab {
    pub fn new() -> Self {
        Self {
            namespaces: None,
            namespace: None,
            namespace_symbols: None,
            symbol: None,
            symbol_tag_keys: None,
            symbol_tag_key: None,
            symbol_tag_values: None,
            symbol_tag_value: None,
            object_info: None,
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

    fn list_to_vec(env: &Environment, list: Tag) -> Vec<String> {
        let core = env.core.as_ref().unwrap();

        let list_str = core.system.write(list, false);
        if core.system.mu().eq(core.nil, list) {
            return vec![];
        }

        let trim = Self::triml(Self::trimr(&list_str)).to_string();
        trim.split(' ')
            .map(|str| str.to_string())
            .collect::<Vec<_>>()
    }

    fn list_to_sorted_vec(env: &Environment, list: Tag) -> Vec<String> {
        let mut sorted_vec = Self::list_to_vec(env, list);

        sorted_vec.sort_by_key(|a| a.to_lowercase());

        sorted_vec
    }

    fn fetch_ns_list(env: &Environment) -> Vec<String> {
        let core = env.core.as_ref().unwrap();
        let list = Core::eval_rstring(&core.system, "(mu:ns-map)".to_string());

        Self::list_to_sorted_vec(env, list)
    }

    fn fetch_symbols_list(env: &Environment, ns: String) -> Vec<String> {
        let core = env.core.as_ref().unwrap();
        let list = Core::eval_rstring(
            &core.system,
            format!("(mu:ns-syms :list {})", ns).to_string(),
        );

        Self::list_to_sorted_vec(env, list)
    }

    fn inspect_repr(env: &Environment, repr: &String) -> String {
        let core = env.core.as_ref().unwrap();
        let inspect_cmd = format!("(eth:inspect-repr {})", repr);
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    fn inspect_repr_tag_keys(env: &Environment, repr: &String) -> String {
        let core = env.core.as_ref().unwrap();
        let inspect_cmd = format!("(eth:inspect-repr-tag-keys {})", repr).to_string();
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    fn inspect_repr_tag_values(env: &Environment, repr: &String) -> String {
        let core = env.core.as_ref().unwrap();
        let inspect_cmd = format!("(eth:inspect-repr-tag-values {})", repr).to_string();
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    fn inspect_symbol(env: &Environment, ns: &str, symbol: &String) -> String {
        let core = env.core.as_ref().unwrap();

        let inspect_cmd = format!("(eth:inspect '{}:{})", Self::triml(ns), symbol).to_string();
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    fn inspect_symbol_tag_keys(env: &Environment, ns: &str, symbol: &String) -> String {
        let core = env.core.as_ref().unwrap();

        let inspect_cmd =
            format!("(eth:inspect-tag-keys '{}:{})", Self::triml(ns), symbol).to_string();
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    fn inspect_symbol_tag_values(env: &Environment, ns: &str, symbol: &String) -> String {
        let core = env.core.as_ref().unwrap();

        let inspect_cmd =
            format!("(eth:inspect-tag-values '{}:{})", Self::triml(ns), symbol).to_string();
        let inspect_str = Core::eval_rstring(&core.system, inspect_cmd);

        core.system.write(inspect_str, false)
    }

    pub fn update(&mut self, env: &Environment, message: InspectorMessage) {
        match message {
            InspectorMessage::SelectSymbol(str) => {
                self.symbol = Some(str);

                self.symbol_tag_keys = Some(
                    Self::trimr(&Self::inspect_symbol_tag_keys(
                        env,
                        self.namespace.as_ref().unwrap(),
                        self.symbol.as_ref().unwrap(),
                    ))
                    .to_string()
                    .split(';')
                    .map(|str| str.to_string())
                    .collect::<Vec<_>>(),
                );

                self.symbol_tag_values = Some(
                    Self::trimr(&Self::inspect_symbol_tag_values(
                        env,
                        self.namespace.as_ref().unwrap(),
                        self.symbol.as_ref().unwrap(),
                    ))
                    .to_string()
                    .split(';')
                    .map(|str| str.to_string())
                    .collect::<Vec<_>>(),
                );

                self.object_info = Some(Self::inspect_symbol(
                    env,
                    self.namespace.as_ref().unwrap(),
                    self.symbol.as_ref().unwrap(),
                ));
            }
            InspectorMessage::SelectSymbolTagKey(str) => {
                let key_offset = self
                    .symbol_tag_keys
                    .as_ref()
                    .unwrap()
                    .iter()
                    .position(|list_str| *list_str == str)
                    .unwrap();

                self.symbol_tag_value =
                    Some(self.symbol_tag_values.as_ref().unwrap()[key_offset].clone());

                self.symbol_tag_key = Some(str.clone());
            }
            InspectorMessage::SelectNamespace(str) => {
                self.namespace = Some(str.clone());
                self.namespace_symbols = Some(Self::fetch_symbols_list(env, str.to_string()));
                self.symbol = None;
                self.symbol_tag_keys = None;
                self.symbol_tag_key = None;
                self.symbol_tag_values = None;
                self.symbol_tag_value = None;
                self.object_info = None;
            }
            InspectorMessage::Refresh => match self.namespaces {
                Some(_) => (),
                None => self.namespaces = Some(Self::fetch_ns_list(env)),
            },
            InspectorMessage::Clear => {
                self.namespace = None;
                self.symbol = None;
                self.namespace_symbols = None;
                self.symbol_tag_keys = None;
                self.symbol_tag_key = None;
                self.symbol_tag_values = None;
                self.symbol_tag_value = None;
                self.object_info = None
            }
        }
    }

    fn namespaces(&self, width: i32, height: i32) -> Element<InspectorMessage> {
        let column = match &self.namespaces {
            Some(symvec) => {
                let selection_list = SelectionList::new_with(
                    symvec,
                    InspectorMessage::SelectNamespace,
                    18.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(250.0));

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
        let column = match &self.namespace_symbols {
            Some(sym_vec) => {
                let selection_list = SelectionList::new_with(
                    sym_vec,
                    InspectorMessage::SelectSymbol,
                    18.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(250.0));

                column!(
                    text(self.namespace.as_ref().unwrap()).size(20),
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
        match &self.object_info {
            Some(descr) => {
                let selection_list = SelectionList::new_with(
                    self.symbol_tag_keys.as_ref().unwrap(),
                    InspectorMessage::SelectSymbolTagKey,
                    18.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(250.0));

                let column = column!(
                    text(format!(
                        "{}:{}",
                        Self::triml(self.namespace.as_ref().unwrap()),
                        self.symbol.as_ref().unwrap()
                    ))
                    .size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                    text(descr).size(16),
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

    fn inspect(&self, env: &Environment, width: i32, height: i32) -> Element<InspectorMessage> {
        let column = match &self.symbol_tag_value {
            Some(repr) => {
                let selection_list = SelectionList::new_with(
                    Self::inspect_repr_tag_keys(env, repr)
                        .split(';')
                        .map(|str| str.to_string())
                        .collect::<Vec<_>>(),
                    InspectorMessage::SelectSymbolTagKey,
                    18.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Fixed(150.0))
                .height(Length::Fixed(150.0));

                column!(
                    text("inspect:".to_string()).size(20),
                    horizontal_rule(1),
                    Space::new(width as u16, 5),
                    //                    text(format!("; {}", repr)).size(16),
                    text(Self::inspect_repr(env, repr)).size(16),
                    selection_list,
                )
            }
            None => {
                column!(
                    text("inspect:".to_string()).size(20),
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
