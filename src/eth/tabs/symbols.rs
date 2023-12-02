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
    SelectSymbol(String),
    SelectNamespace(String),
    Refresh,
}

pub struct SymbolsTab {
    namespaces: Option<Vec<String>>,
    selected_ns: Option<String>,
    symbols: Option<Vec<String>>,
    selected_symbol: Option<String>,
    describe: Option<String>,
}

impl SymbolsTab {
    pub fn new() -> Self {
        Self {
            namespaces: None,
            selected_ns: None,
            symbols: None,
            selected_symbol: None,
            describe: None,
        }
    }

    fn fetch_ns_list(env: &Environment) -> Vec<String> {
        fn rem_first_and_last(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        }

        let ns_list = Core::eval_rstring(
            &env.core.as_ref().unwrap().system,
            "(mu:ns-map)".to_string(),
        );
        let ns_str = env.core.as_ref().unwrap().system.write(ns_list, false);
        let trim = rem_first_and_last(&ns_str).to_string();
        let slice = &trim.split(' ').collect::<Vec<&str>>();

        let mut ns = vec![];

        for ns_ in slice {
            ns.push(ns_.to_string())
        }

        ns
    }

    fn fetch_symbols_list(env: &Environment, ns: String) -> Vec<String> {
        fn rem_first_and_last(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.next_back();
            chars.as_str()
        }

        let syms_cmd = format!("(mu:ns-syms :list {})", ns).to_string();
        let sym_list = Core::eval_rstring(&env.core.as_ref().unwrap().system, syms_cmd);
        let sym_str = env.core.as_ref().unwrap().system.write(sym_list, false);
        let trim = rem_first_and_last(&sym_str).to_string();
        let slice = &trim.split(' ').collect::<Vec<&str>>();

        let mut syms = vec![];

        for sym in slice {
            syms.push(sym.to_string())
        }

        syms
    }

    fn describe_symbol(_env: &Environment, ns: &str, symbol: &String) -> String {
        fn rem_first(value: &str) -> &str {
            let mut chars = value.chars();
            chars.next();
            chars.as_str()
        }

        // println!("describe: {}:{}", rem_first(ns), symbol);

        // let describe_cmd =
        format!("(core:describe '{}:{})", rem_first(ns), symbol).to_string()
        // ;
        // let sym_list = Core::eval_rstring(&env.core.as_ref().unwrap().system, describe_cmd);
    }

    pub fn update(&mut self, env: &Environment, message: SymbolsMessage) {
        match message {
            SymbolsMessage::SelectSymbol(str) => {
                self.selected_symbol = Some(str);
                self.describe = Some(Self::describe_symbol(
                    env,
                    self.selected_ns.as_ref().unwrap(),
                    self.selected_symbol.as_ref().unwrap(),
                ));
            }
            SymbolsMessage::SelectNamespace(str) => {
                self.selected_ns = Some(str.clone());
                self.symbols = Some(Self::fetch_symbols_list(env, str.to_string()));
            }
            SymbolsMessage::Refresh => match self.namespaces {
                Some(_) => (),
                None => self.namespaces = Some(Self::fetch_ns_list(env)),
            },
        }
    }

    fn namespaces(&self, width: i32, height: i32) -> Element<SymbolsMessage> {
        match &self.namespaces {
            Some(symvec) => {
                let selection_list = SelectionList::new_with(
                    symvec,
                    SymbolsMessage::SelectNamespace,
                    16.0,
                    1.0,
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
            None => {
                let column = column!(
                    text("namespaces:".to_string()).size(20),
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

    fn namespace_symbols(
        &self,
        _: &Environment,
        width: i32,
        height: i32,
    ) -> Element<SymbolsMessage> {
        match &self.symbols {
            Some(symvec) => {
                let selection_list = SelectionList::new_with(
                    symvec,
                    SymbolsMessage::SelectSymbol,
                    16.0,
                    1.0,
                    SelectionListStyles::Default,
                )
                .width(Length::Shrink)
                .height(Length::Fixed(100.0));

                let column = column!(
                    text("symbols:".to_string()).size(20),
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
            None => {
                let column = column!(
                    text("symbols:".to_string()).size(20),
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

    fn symbol(&self, _: &Environment, width: i32, height: i32) -> Element<SymbolsMessage> {
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
                    text(descr).size(20),
                );

                // let lines = self.describe.as_ref().unwrap().split('\n').collect::<Vec<&str>>();
                // for line in lines {
                //     column.push(text(line.to_string()).size(20));
                // }

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
