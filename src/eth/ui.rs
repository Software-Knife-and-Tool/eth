// SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
// SPDX-License-Identifier: MIT

// eth tab bar ui
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]

use {
    super::super::{Core, Environment},
    crate::eth::tabs::{
        info::{InfoMessage, InfoTab},
        listener::{ListenerMessage, ListenerTab},
        namespaces::{NamespacesMessage, NamespacesTab},
        scratchpad::{ScratchpadMessage, ScratchpadTab},
    },
    iced::{
        alignment::{Horizontal, Vertical},
        executor, subscription, theme,
        widget::{column, container, horizontal_rule, row, text, Column, Container, Row, Text},
        Alignment, Application, Command, Element, Event, Length, Subscription, Theme,
    },
    iced_aw::{TabLabel, Tabs},
    mu::Mu,
};

#[derive(Debug, Default)]
pub struct StatusBar {
    config_path: String,
}

// status bar
impl StatusBar {
    pub fn new(_env: &Environment) -> Self {
        let config_path = "~/.config/eth/config.json".to_string();

        StatusBar { config_path }
    }

    pub fn view(&self, _filter: String) -> Element<Message> {
        /*
                let filter = text(format!("filter: {}", filter)).size(20);
                let host_path = text(self.host_path.clone()).size(20);
                let buttons = row![
                    iced::widget::button(text("clear".to_string()).size(13))
                        .height(28)
                        .style(theme::Button::Primary)
                        .on_press(Message::Clear),
                    iced::widget::button(text("refresh".to_string()).size(13))
                        .height(28)
                        .style(theme::Button::Primary)
                        .on_press(Message::Poll)
                ]
                .spacing(8);
        */

        let content = Row::new()
            .align_items(Alignment::Center)
            .spacing(6)
            .push(text(&self.config_path).width(Length::Fill));
        //            .push(filter.width(Length::Fill))
        //            .push(buttons.width(Length::Shrink));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// application tab bar
pub struct Ui {
    active_tab: usize,
    env: Environment,
    info_tab: InfoTab,
    listener_tab: ListenerTab,
    namespaces_tab: NamespacesTab,
    poll_interval_secs: u64,
    version: String,
    scratchpad_tab: ScratchpadTab,
}

#[derive(Clone, Debug)]
pub enum Message {
    Scratchpad(ScratchpadMessage),
    EventOccurred(Event),
    Info(InfoMessage),
    Listener(ListenerMessage),
    Namespaces(NamespacesMessage),
    TabSelected(usize),
}

impl Application for Ui {
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Environment;
    type Message = Message;

    fn new(env: Environment) -> (Ui, Command<Message>) {
        let tab_bar = Ui {
            version: "0.0.3".to_string(),
            active_tab: 0,
            scratchpad_tab: ScratchpadTab::new(),
            info_tab: InfoTab::new(),
            listener_tab: ListenerTab::new(),
            namespaces_tab: NamespacesTab::new(),
            poll_interval_secs: 10,
            env,
        };

        let (opt, _) = &tab_bar.env.config;
        match opt {
            Some(how) => {
                if *how {
                    tab_bar
                        .info_tab
                        .log("eth: config directory missing, using default config".to_string())
                } else {
                    tab_bar
                        .info_tab
                        .log("eth: using supplied config".to_string())
                }
            }
            None => tab_bar
                .info_tab
                .log("eth: config file missing or damaged, using default".to_string()),
        }

        tab_bar
            .info_tab
            .log(format!("mu: local runtime v{}", Mu::VERSION));

        let itab = &tab_bar.info_tab;

        if tab_bar.env.core.as_ref().unwrap().init_loaded {
            itab.log("core: init environment active".to_string());
        } else {
            itab.log("core: init environment missing or damaged".to_string());
            itab.log("core: many UI things will not work".to_string())
        }

        (tab_bar, Command::none())
    }

    fn title(&self) -> String {
        format!("eth {}", self.version)
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::TabSelected(selected) => self.active_tab = selected,
            Message::EventOccurred(event) => {
                self.listener_tab
                    .update(&self.env, ListenerMessage::EventOccurred(event));
            }
            Message::Listener(message) => self.listener_tab.update(&self.env, message),
            Message::Scratchpad(message) => self.scratchpad_tab.update(message),
            Message::Info(message) => self.info_tab.update(message),
            Message::Namespaces(message) => self.namespaces_tab.update(message),
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Tabs::new(self.active_tab, Message::TabSelected)
            .push(self.info_tab.tab_label(), self.info_tab.view(&self.env))
            .push(
                self.scratchpad_tab.tab_label(),
                self.scratchpad_tab.view(&self.env),
            )
            .push(
                self.listener_tab.tab_label(),
                self.listener_tab.view(&self.env),
            )
            .push(
                self.namespaces_tab.tab_label(),
                self.namespaces_tab.view(&self.env),
            )
            .into()
    }
}

pub trait Tab {
    const HEADER_SIZE: u16 = 32;
    const TAB_PADDING: u16 = 16;

    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(Self::HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(Self::TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}
