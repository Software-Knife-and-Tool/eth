//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// system term panel
//
#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_imports)]

use {
    iced::{
        alignment, executor,
        widget::{button, checkbox, container, text, Column},
        window, Alignment, Command, Element, Length, Settings, Subscription, Theme,
    },
    iced_native::Event,
    std::sync::RwLock,
};

#[derive(Debug, Default)]
struct Events {
    last: Vec<iced_native::Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
}

pub struct SysTty {
    text: RwLock<Vec<String>>,
    events: Events,
}

impl SysTty {
    pub fn new() -> Self {
        SysTty {
            text: RwLock::new(Vec::<String>::new()),
            events: Events {
                last: Vec::<iced_native::Event>::new(),
                enabled: true,
            },
        }
    }

    pub fn writeln(&self, message: String) {
        let mut text = self.text.write().unwrap();

        text.push(message);
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        print!(".");
        match message {
            Message::EventOccurred(event) if self.events.enabled => {
                self.events.last.push(event);

                if self.events.last.len() > 5 {
                    let _ = self.events.last.remove(0);
                }

                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => {
                self.events.enabled = enabled;

                Command::none()
            }
            Message::Exit => window::close(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    pub fn contents(&self) -> Option<String> {
        self.writeln("> ".to_string());
        let text = self.text.read().unwrap();

        if text.len() == 0 {
            None
        } else {
            Some(text.join("\n"))
        }
    }

    fn view(&self) -> Element<Message> {
        let events = Column::with_children(
            self.events
                .last
                .iter()
                .map(|event| text(format!("{event:?}")).size(40))
                .map(Element::from)
                .collect(),
        );

        let toggle = checkbox(
            "Listen to runtime events",
            self.events.enabled,
            Message::Toggled,
        );

        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
