//  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

// scratchpad panel
//
#![allow(clippy::new_without_default)]
#![allow(unused_imports)]
use {
    super::super::{
        super::{Core, Environment},
        ui::{Message, Tab},
    },
    super::eth::CoreButton,
    iced::{
        alignment::{self, Horizontal, Vertical},
        executor, subscription, theme,
        widget::{container, horizontal_rule, row, text, Column, Container, Row, Scrollable, Text},
        window, Alignment, Application, Command, Element, Event, Length, Renderer, Subscription,
        Theme,
    },
    iced_aw::{grid, tab_bar::TabLabel, Grid},
    std::sync::RwLock,
};

// components
#[derive(Debug, Default)]
pub struct ControlInfo {
    image: RwLock<String>,
    lines: RwLock<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl ControlInfo {
    pub fn new(rows: usize, cols: usize) -> Self {
        ControlInfo {
            image: RwLock::new(String::new()),
            lines: RwLock::new(vec![String::new(); rows]),
            rows,
            cols,
        }
    }

    fn collapse(&self) {
        let mut image = self.image.write().unwrap();
        let lines = self.lines.read().unwrap();

        let mut img = String::new();

        for line in &lines[0..self.rows - 1] {
            if line.is_empty() {
                img.push_str(" \n");
            } else {
                img.push_str(line);
                img.push('\n')
            }
        }

        if lines[self.rows - 1].is_empty() {
            img.push(' ');
        } else {
            img.push_str(&lines[self.rows - 1]);
        }

        *image = img
    }

    pub fn clear(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            *lines = vec![String::new(); self.rows]
        }

        self.collapse()
    }

    pub fn scroll(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            lines.remove(0);
            lines.push(String::new())
        }

        self.collapse()
    }

    pub fn backspace(&self) {
        {
            let mut lines = self.lines.write().unwrap();

            if !lines[self.rows - 1].is_empty() {
                lines[self.rows - 1].pop().unwrap();
            }
        }

        self.collapse()
    }

    pub fn write_char(&self, ch: char) {
        {
            let mut lines = self.lines.write().unwrap();

            lines[self.rows - 1].push(ch)
        }

        self.collapse()
    }

    pub fn write(&self, str: String) {
        {
            let mut lines = self.lines.write().unwrap();

            lines[self.rows - 1].push_str(&str)
        }

        self.collapse()
    }

    pub fn contents(&self) -> String {
        let image = self.image.read().unwrap();

        image.clone()
    }

    pub fn content(&self) -> Element<'_, ScratchpadMessage> {
        let content: Element<'_, ScratchpadMessage> =
            Column::new().push(text(self.contents())).into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Default)]
pub struct ControlGroups {
    cols: usize,
}

impl ControlGroups {
    pub fn new(cols: usize) -> Self {
        ControlGroups { cols }
    }

    pub fn content(&self, groups: &[String]) -> Element<'_, ScratchpadMessage> {
        let grid_spacer = "                                 ";

        let mut group_grid = Grid::with_columns(self.cols);

        for (id, group) in groups.iter().enumerate() {
            if id % self.cols == 0 {
                for _ in 0..self.cols {
                    group_grid.insert(text(grid_spacer));
                }
            }

            group_grid.insert(
                iced::widget::button(text(group))
                    .height(30)
                    .style(theme::Button::Primary)
                    .on_press(ScratchpadMessage::GroupPress(id)),
            );
        }

        let content: Element<'_, ScratchpadMessage> = Column::new()
            .align_items(Alignment::Start)
            .spacing(20)
            .push(Scrollable::new(group_grid))
            .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug, Default)]
pub struct ScratchpadGrid {
    cols: usize,
    scratchpad: RwLock<Vec<CoreButton>>,
}

impl ScratchpadGrid {
    pub fn new(cols: usize) -> Self {
        let scratchpad = RwLock::new(Vec::new());
        ScratchpadGrid { cols, scratchpad }
    }

    pub fn add_control(&self, button: CoreButton) {
        let mut scratchpad = self.scratchpad.write().unwrap();

        scratchpad.push(button)
    }

    pub fn content(&self, filter: String) -> Element<'_, ScratchpadMessage> {
        let grid_spacer = "                                 ";

        let scratchpad = self.scratchpad.read().unwrap();

        let mut scratchpad_grid = Grid::with_columns(self.cols);
        for (nth, control) in scratchpad
            .iter()
            .filter(|scratchpad| {
                if filter.is_empty() {
                    true
                } else {
                    scratchpad.group == *filter
                }
            })
            .enumerate()
        {
            if nth % self.cols == 0 {
                for _ in 0..self.cols {
                    scratchpad_grid.insert(text(grid_spacer));
                }
            }

            let nth_control = if filter.is_empty() {
                nth
            } else {
                scratchpad
                    .iter()
                    .position(|group| group.group == control.group)
                    .unwrap()
            };

            scratchpad_grid.insert(
                iced::widget::button(text(&control.label))
                    .style(theme::Button::Primary)
                    .on_press(ScratchpadMessage::ControlPress(nth_control)),
            );
        }

        let content: Element<'_, ScratchpadMessage> = Scrollable::new(
            Column::new()
                .align_items(Alignment::Start)
                .spacing(20)
                .push(scratchpad_grid),
        )
        .into();

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// tab
#[derive(Debug, Clone)]
pub enum ScratchpadMessage {
    GroupPress(usize),
    ControlPress(usize),
}

pub struct ScratchpadTab {
    cgroup: RwLock<String>,
    cgroups: ControlGroups,
    cgroup_labels: Vec<String>,
    scratchpad_grid: ScratchpadGrid,
    scratchpad: Vec<CoreButton>,
    control_info: ControlInfo,
    poll_interval_secs: u64,
}

impl ScratchpadTab {
    pub fn new() -> Self {
        ScratchpadTab {
            cgroup: RwLock::new(String::new()),
            cgroups: ControlGroups::new(5),
            cgroup_labels: Vec::new(),
            scratchpad_grid: ScratchpadGrid::new(5),
            scratchpad: Vec::new(),
            control_info: ControlInfo::new(12, 30),
            poll_interval_secs: 60,
        }
    }

    pub fn update(&mut self, message: ScratchpadMessage) {
        match message {
            ScratchpadMessage::GroupPress(_) => (),
            ScratchpadMessage::ControlPress(_) => (),
        }
    }

    pub fn view(&self, _env: &Environment) -> iced_native::Element<'_, Message, Renderer> {
        let grid_spacer = "                                 ";

        let mut group_grid = Grid::with_columns(self.cgroups.cols);

        for (id, group) in self.cgroup_labels.iter().enumerate() {
            if id % self.cgroups.cols == 0 {
                for _ in 0..self.scratchpad_grid.cols {
                    group_grid.insert(text(grid_spacer));
                }
            }

            group_grid.insert(
                iced::widget::button(text(group))
                    .height(30)
                    .style(theme::Button::Primary)
                    .on_press(ScratchpadMessage::GroupPress(id)),
            );
        }

        let scratchpad = row![
            container(
                Column::new()
                    .push(Text::new("control info").size(20))
                    .push(horizontal_rule(1))
                    .push(self.control_info.content())
                    .width(600)
                    .height(400)
            )
            .width(Length::Fill)
            .height(Length::Fill),
            container(
                Column::new()
                    .push(Text::new("scratchpad filter").size(20))
                    .push(horizontal_rule(1))
                    .push(Column::new().push(group_grid).width(400).height(150))
                    .push(Text::new("scratchpad grid").size(20))
                    .push(horizontal_rule(1))
                    .push(
                        Column::new()
                            .push(self.scratchpad_grid.content(String::new()))
                            .width(400)
                            .height(150)
                    )
            )
            .width(Length::Fill)
            .height(Length::Fill),
        ];

        let content: Element<'_, ScratchpadMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(600)
                .padding(20)
                .spacing(10)
                .push(scratchpad)
                .push(horizontal_rule(1)),
        )
        .into();

        content.map(Message::Scratchpad)
    }

    /*
    pub fn view_(&self, _env: &Environment) -> Element<'_, Message, Renderer> {
        let content: Element<'_, ScratchpadMessage> = Container::new(
            Column::new()
                .align_items(Alignment::Start)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(Text::new("scratchpad").size(20))
                .push(Text::new("foo").size(30))
                .push(horizontal_rule(1)),
        )
        .into();

        content.map(Message::Scratchpad)
    }
     */
}

impl Tab for ScratchpadTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Eth::Scratchpad")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("scratchpad".to_string())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        panic!();
    }
}
