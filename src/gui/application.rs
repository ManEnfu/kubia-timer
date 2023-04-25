use iced::{
    alignment,
    keyboard::{self, KeyCode},
    subscription, time, widget, Alignment, Application, Command, Event, Length, Subscription,
};
use std::time::{Duration, Instant, SystemTime};

use crate::data;

use crate::tangible;

pub use preferences::Preferences;

mod preferences;

pub struct KTApplication {
    preferences: Preferences,

    solve_time: data::SolveTime,
    link_to_last_solve: bool,
    last_pressed: Instant,
    state: State,
    session: data::Session,
}

pub enum State {
    Idle { pressed: bool },
    Ready,
    Timing { last_tick: Instant },
    Finished,
}

#[derive(Debug, Clone)]
pub enum ButtonType {
    SetOk,
    SetPlus2,
    SetDnf,

    SolveSummary { index: usize },
    DeleteSolve { index: usize },
}

#[derive(Debug, Clone)]
pub enum Message {
    TriggerPress,
    TriggerRelease,
    TriggerTimeout,
    Tick(Instant),

    PenaltySelected(Option<data::Penalty>),
    ButtonPressed(ButtonType),

    SolveSelected { index: usize },
    ThemeSelected(tangible::Theme),

    Todo,
}

// pub enum SolveEditMessage {
//     SolveTimeChanged
// }

impl iced::Application for KTApplication {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = tangible::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                preferences: Preferences::new(),

                solve_time: data::SolveTime::default(),
                link_to_last_solve: false,
                last_pressed: Instant::now(),
                state: State::Idle { pressed: false },
                session: data::Session::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kubia Timer")
    }

    fn theme(&self) -> Self::Theme {
        self.preferences.theme
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        const PRESS_START_INTERVAL: Duration = Duration::from_millis(500);

        let command = match message {
            Message::TriggerPress => {
                self.last_pressed = Instant::now();
                match &mut self.state {
                    State::Idle { pressed } => {
                        let start_press = !*pressed;
                        *pressed = true;
                        if start_press {
                            Command::perform(async_std::task::sleep(PRESS_START_INTERVAL), |()| {
                                Message::TriggerTimeout
                            })
                        } else {
                            Command::none()
                        }
                    }
                    State::Timing { last_tick: _ } => {
                        self.session.add_solve(data::Solve {
                            time: self.solve_time,
                            timestamp: SystemTime::now(),
                            scramble: "".to_string(),
                        });
                        self.link_to_last_solve = true;
                        self.state = State::Finished;
                        Command::none()
                    }
                    _ => Command::none(),
                }
            }
            Message::TriggerRelease => {
                match &mut self.state {
                    State::Idle { pressed } => {
                        *pressed = false;
                    }
                    State::Ready => {
                        self.state = State::Timing {
                            last_tick: Instant::now(),
                        };
                    }
                    State::Finished => {
                        self.state = State::Idle { pressed: false };
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::TriggerTimeout => {
                match &mut self.state {
                    State::Idle { pressed } => {
                        if *pressed && Instant::now() - self.last_pressed > PRESS_START_INTERVAL {
                            self.solve_time = data::SolveTime::default();
                            self.state = State::Ready;
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::Tick(now) => {
                match &mut self.state {
                    State::Timing { last_tick } => {
                        self.solve_time.time += now - *last_tick;
                        *last_tick = now;
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::PenaltySelected(penalty) => {
                self.solve_time.penalty = penalty;
                if self.link_to_last_solve {
                    if let Some(s) = self.session.last_solve_mut() {
                        s.time = self.solve_time;
                        self.session.update_statistics_last();
                    }
                }
                Command::none()
            }
            Message::ThemeSelected(theme) => {
                self.preferences.theme = theme;
                Command::none()
            }
            _ => Command::none(),
        };

        command
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let kbs = subscription::events_with(|e, _s| {
            if let Event::Keyboard(ke) = e {
                match ke {
                    keyboard::Event::KeyPressed {
                        key_code: KeyCode::Space,
                        ..
                    } => Some(Message::TriggerPress),
                    keyboard::Event::KeyReleased {
                        key_code: KeyCode::Space,
                        ..
                    } => Some(Message::TriggerRelease),
                    _ => None,
                }
            } else {
                None
            }
        });

        match self.state {
            State::Timing { last_tick: _ } => Subscription::batch([
                kbs,
                time::every(Duration::from_millis(10)).map(Message::Tick),
            ]),
            _ => kbs,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        iced_lazy::responsive(move |size| {
            let compact = size.width <= 600.0;

            let content = if compact {
                let mut column = widget::Column::new();
                column = column.push(self.center_timer());
                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    column = column
                        .push(widget::horizontal_rule(1))
                        .push(self.bottombar());
                }

                iced::Element::from(column)
            } else {
                let mut row = widget::Row::new();
                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    row = row.push(self.sidebar()).push(widget::vertical_rule(1));
                }
                row = row.push(self.center_timer());

                iced::Element::from(row)
            };

            // let mut winbox = widget::Column::new();
            // if !matches!(self.state, State::Ready | State::Timing { .. }) {
            // }
            // winbox = winbox.push(content);
            // winbox.into()
            widget::column![
                self.headerbar(),
                content,
            ]
            .into()
        })
        .into()
    }
}

impl KTApplication {
    fn headerbar(&self) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        widget::column![
            widget::row![
                widget::pick_list(
                    &tangible::Theme::ALL[..],
                    Some(self.preferences.theme),
                    Message::ThemeSelected,
                )
                .padding([4, 8])
                .width(Length::Shrink)
                .placeholder("--"),
            ]
            .spacing(4)
            .padding(4),
            widget::horizontal_rule(1),
        ]
        .into()
    }

    fn center_timer(&self) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        iced_lazy::responsive(move |size| {
            let compact = size.width <= 450.0;
            let duration_text_font_size = if compact
                || self
                    .solve_time
                    .recorded_time()
                    .map(|t| t.as_secs() > 600)
                    .unwrap_or(false)
            {
                80.0
            } else {
                120.0
            };

            let theme = self.theme();
            let palette = theme.palette();
            let duration_text_color = match self.state {
                State::Idle { pressed: true } | State::Finished => palette.destructive.base,
                State::Ready => palette.success.base,
                _ => palette.view.fg,
            };

            let duration_text = if let Some(duration) = self.solve_time.recorded_time() {
                let seconds = duration.as_secs();
                widget::row![
                    widget::text(if seconds >= 60 {
                        format!("{:0>1}:{:0>2}", seconds / 60, seconds % 60,)
                    } else {
                        format!("{:0>1}", seconds,)
                    })
                    .style(duration_text_color)
                    .size(duration_text_font_size),
                    widget::text(format!(
                        ".{:0>2}{}",
                        duration.subsec_millis() / 10,
                        if self.solve_time.is_plus2() { "+" } else { "" }
                    ))
                    .style(duration_text_color)
                    .size(duration_text_font_size * 0.75),
                ]
                .align_items(Alignment::End)
            } else {
                widget::row![widget::text("DNF")
                    .style(duration_text_color)
                    .size(duration_text_font_size),]
            };

            let center_content = {
                let mut center_content = widget::column![duration_text,]
                    .spacing(16)
                    .align_items(Alignment::Center);

                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    if self.link_to_last_solve {
                        center_content = center_content.push(self.penalty_selector());
                    }
                    center_content = center_content.push(
                        widget::column![
                            widget::row![
                                widget::text("Ao5")
                                    .width(Length::FillPortion(1))
                                    .horizontal_alignment(alignment::Horizontal::Right),
                                widget::text(
                                    self.session
                                        .last_ao5()
                                        .map_or("--".to_string(), |s| s.to_string()),
                                )
                                .width(Length::FillPortion(1))
                                .horizontal_alignment(alignment::Horizontal::Left),
                            ]
                            .spacing(8)
                            .width(Length::Fixed(200.0)),
                            widget::row![
                                widget::text("Ao12")
                                    .width(Length::FillPortion(1))
                                    .horizontal_alignment(alignment::Horizontal::Right),
                                widget::text(
                                    self.session
                                        .last_ao12()
                                        .map_or("--".to_string(), |s| s.to_string()),
                                )
                                .width(Length::FillPortion(1))
                                .horizontal_alignment(alignment::Horizontal::Left),
                            ]
                            .spacing(8)
                            .width(Length::Fixed(200.0)),
                        ]
                        .spacing(8),
                    )
                }

                // center_content.push(
                //     widget::pick_list(
                //         &tangible::Theme::ALL[..],
                //         Some(self.preferences.theme),
                //         Message::ThemeSelected,
                //     )
                //     .padding([4, 8])
                //     .width(Length::Shrink)
                //     .placeholder("--"),
                // )
                center_content
            };

            widget::container(center_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        })
        .into()
    }

    fn penalty_button(
        &self,
        label: &str,
        penalty: Option<data::Penalty>,
    ) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        let label = widget::text(label).horizontal_alignment(alignment::Horizontal::Center);

        let style = if self.solve_time.penalty == penalty {
            self.theme().palette().selector_active.into()
        } else {
            // tangible::theme::Button::Flat
            self.theme().palette().selector.into()
        };

        widget::button(label)
            .style(style)
            .padding(4)
            .on_press(Message::PenaltySelected(penalty))
            .width(Length::FillPortion(1))
            .into()
    }

    fn penalty_selector(&self) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        let row = widget::row![
            self.penalty_button("OK", None),
            self.penalty_button("+2", Some(data::Penalty::Plus2)),
            self.penalty_button("DNF", Some(data::Penalty::Dnf)),
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .width(Length::Fixed(200.0));

        widget::container(row)
            // .style(tangible::theme::Container::Solid(
            //     // tangible::theme::NamedColor::Neutral,
            //     self.theme().palette().selector.into()
            // ))
            .style(self.theme().palette().selector)
            .padding(4)
            .into()
    }

    fn sidebar(&self) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        if self.session.get_n_solves() > 0 {
            let times_column = widget::Column::with_children(
                self.session
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(i, se)| {
                        let row = widget::row![
                            widget::text(se.solve.time)
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::FillPortion(1)),
                            widget::text(se.ao5.map_or("--".to_string(), |ao5| ao5.to_string()))
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::FillPortion(1)),
                            widget::text(se.ao12.map_or("--".to_string(), |ao12| ao12.to_string()))
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .width(Length::FillPortion(1)),
                        ]
                        .spacing(4);

                        widget::button(row)
                            .padding([4, 8])
                            .style(tangible::theme::Button::Flat)
                            .on_press(Message::SolveSelected { index: i })
                            .into()
                    })
                    .collect(),
            )
            .spacing(4)
            .padding(8)
            .align_items(Alignment::Start)
            .width(Length::Fixed(300.0));

            widget::scrollable(times_column)
                .vertical_scroll(
                    widget::scrollable::Properties::new()
                        .width(4.0)
                        .margin(4.0)
                        .scroller_width(4.0),
                )
                .into()
        } else {
            let content = widget::column![
                widget::text("No Solves").size(32.0),
                widget::text("Add solve by starting the timer.").style(tangible::theme::Text::Dim),
            ]
            .spacing(8)
            .align_items(Alignment::Center);

            widget::container(content)
                .width(Length::Fixed(300.0))
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }

    fn bottombar(&self) -> iced::Element<'_, Message, iced::Renderer<tangible::Theme>> {
        let times_row = widget::Row::with_children(
            self.session
                .iter()
                .enumerate()
                .map(|(i, se)| {
                    widget::button(widget::text(se.solve.time))
                        .padding([4, 8])
                        .style(tangible::theme::Button::Flat)
                        .on_press(Message::SolveSelected { index: i })
                        .into()
                })
                .collect(),
        )
        .spacing(4)
        .padding(4)
        .align_items(Alignment::Start);

        widget::scrollable(times_row)
            .horizontal_scroll(
                widget::scrollable::Properties::new()
                    .width(4.0)
                    .margin(4.0)
                    .scroller_width(4.0),
            )
            .into()
    }
}
