//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// listener panel
//
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::single_match)]
#![allow(unused_imports)]
use {
    super::{
        super::{
            super::{Core, Environment},
            ui::{Message, Tab},
        },
        eth::Eth,
        tty::{Tty, TtyBuilder},
    },
    iced::{
        alignment::{Horizontal, Vertical},
        executor,
        keyboard::Event::CharacterReceived,
        subscription,
        widget::{container, horizontal_rule, text, Column, Container, Image, Slider, Text},
        window, Alignment, Application, Command, Element, Event, Length, Renderer, Subscription,
        Theme,
    },
    iced_aw::tab_bar::TabLabel,
    mu::{Condition, Exception, Mu, Result, System, Tag},
};

#[derive(Debug, Clone)]
pub enum ListenerMessage {
    EventOccurred(Event),
}

pub struct ListenerTab {
    command: String,
    tty: Tty,
}

type CoreResult<T> = std::result::Result<T, Exception>;

impl ListenerTab {
    pub fn new() -> Self {
        let tty = TtyBuilder::new().rows(19).cursor('_').build();

        tty.write_string("eth> ".to_string());

        ListenerTab {
            command: String::new(),
            tty,
        }
    }

    pub fn eval(&self, env: &Environment, expr: &String) -> CoreResult<String> {
        let mu = env.core.as_ref().unwrap().system.mu();
        let rstream = env.core.as_ref().unwrap().eval_stream;

        match mu.read_string(expr.to_string()) {
            Ok(form) => match mu.compile(form) {
                Ok(form) => match mu.eval(form) {
                    Ok(value) => match mu.write(value, false, rstream) {
                        Ok(_) => match mu.get_string(rstream) {
                            Ok(string) => {
                                Eth::run(env);
                                Ok(string)
                            }
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn error(env: &Environment, ex: Exception) -> String {
        let system = &env.core.as_ref().unwrap().system;

        system.error(ex)
    }

    pub fn update(&mut self, env: &Environment, message: ListenerMessage) {
        match message {
            ListenerMessage::EventOccurred(event) => match event {
                Event::Keyboard(key_event) => match key_event {
                    CharacterReceived(ch) => match ch {
                        '\r' | '\n' => {
                            self.tty.scroll();
                            match self.eval(env, &self.command) {
                                Ok(string) => self.tty.write_string(string),
                                Err(e) => self.tty.write_string(Self::error(env, e)),
                            }
                            self.tty.scroll();
                            self.tty.write_string("core> ".to_string());

                            self.command.clear();
                        }
                        '\u{c}' => {
                            self.command.clear();
                            self.tty.clear();
                            self.tty.write_string("core> ".to_string());
                        }
                        '\u{8}' => {
                            if !self.command.is_empty() {
                                self.command.pop().unwrap();
                            }
                            self.tty.backspace();
                        }
                        _ => {
                            self.command.push(ch);
                            self.tty.write_char(ch);
                        }
                    },
                    _ => (),
                },
                _ => (),
            },
        }
    }

    pub fn subscription(&self) -> Subscription<ListenerMessage> {
        println!("subscribe");
        subscription::events().map(ListenerMessage::EventOccurred)
    }

    pub fn view(&self, _env: &Environment) -> Element<'_, Message, Renderer> {
        let content: Element<'_, ListenerMessage> = Container::new(
            Column::new()
                .max_width(800)
                .padding(20)
                .spacing(10)
                .push(text("listener".to_string()).size(20))
                .push(horizontal_rule(1))
                .push(Column::new().push(text(self.tty.contents())).height(500))
                .push(horizontal_rule(1))
                .width(800),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Top)
        .into();

        content.map(Message::Listener)
    }
}

impl Tab for ListenerTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Eth::Core")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("listener".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
