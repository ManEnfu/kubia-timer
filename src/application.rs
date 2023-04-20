use iced::{
    alignment,
    keyboard::{self, KeyCode},
    subscription, theme, time, widget, Alignment, Color, Command, Event, Length, Subscription,
};
use std::time::{Duration, Instant, SystemTime};

use crate::data::{self, Session, Solve, SolveTime};

pub struct KubiaTimer {
    solve_time: data::SolveTime,
    link_to_last_solve: bool,
    last_pressed: Instant,
    state: State,
    session: Session,
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
    Tick(Instant),

    PenaltySelected(Option<data::Penalty>),
    ButtonPressed(ButtonType),

    Todo,
}

impl iced::Application for KubiaTimer {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                solve_time: data::SolveTime::default(),
                link_to_last_solve: false,
                last_pressed: Instant::now(),
                state: State::Idle { pressed: false },
                session: Session::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kubia Timer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        const PRESS_START_INTERVAL: Duration = Duration::from_millis(500);

        match message {
            Message::TriggerPress => {
                self.last_pressed = Instant::now();
            }
            Message::PenaltySelected(penalty) => {
                self.solve_time.penalty = penalty;
                if self.link_to_last_solve {
                    if let Some(s) = self.session.last_solve_mut() {
                        s.time = self.solve_time;
                        self.session.update_statistics_last();
                    }
                }
            }
            _ => {}
        }

        match &mut self.state {
            State::Idle { pressed } => match message {
                Message::TriggerPress => {
                    *pressed = true;
                }
                Message::TriggerRelease => {
                    *pressed = false;
                }
                Message::Tick(now) => {
                    if *pressed && now - self.last_pressed > PRESS_START_INTERVAL {
                        self.state = State::Ready;
                        // self.duration = Duration::new(0, 0);
                        self.solve_time = data::SolveTime::default();
                    }
                }
                _ => {}
            },

            State::Ready => match message {
                Message::TriggerRelease => {
                    self.state = State::Timing {
                        last_tick: Instant::now(),
                    };
                }
                _ => {}
            },

            State::Timing { last_tick } => match message {
                Message::TriggerPress => {
                    log::info!("Solve: {}", SolveTime::new(self.solve_time.time, None));
                    self.session.add_solve(Solve {
                        time: self.solve_time,
                        timestamp: SystemTime::now(),
                        scramble: "".to_string(),
                    });
                    self.link_to_last_solve = true;
                    self.state = State::Finished;
                }
                Message::Tick(now) => {
                    self.solve_time.time += now - *last_tick;
                    *last_tick = now;
                }
                _ => {}
            },

            State::Finished => match message {
                Message::TriggerRelease => self.state = State::Idle { pressed: false },
                _ => {}
            },
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Subscription::none()
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

        Subscription::batch([
            kbs,
            time::every(Duration::from_millis(10)).map(Message::Tick),
        ])
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        iced_lazy::responsive(move |size| {
            let compact = size.width <= 600.0;

            if compact {
                let mut column = widget::Column::new();
                column = column.push(self.center_timer());
                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    column = column
                        .push(widget::horizontal_rule(1))
                        .push(self.bottombar());
                }

                column.into()
            } else {
                let mut row = widget::Row::new();
                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    row = row.push(self.sidebar()).push(widget::vertical_rule(1));
                }
                row = row.push(self.center_timer());

                row.into()
            }
        })
        .into()
    }
}

impl KubiaTimer {
    fn center_timer(&self) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
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
            let averages_text_font_size = if compact { 20.0 } else { 30.0 };

            let duration_text_color = match self.state {
                State::Idle { pressed: true } => Color::new(1.0, 0.0, 0.0, 1.0),
                State::Finished => Color::new(1.0, 0.0, 0.0, 1.0),
                State::Ready => Color::new(0.0, 1.0, 0.0, 1.0),
                _ => Color::new(0.0, 0.0, 0.0, 1.0),
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
                    .spacing(12)
                    .align_items(Alignment::Center);

                if !matches!(self.state, State::Ready | State::Timing { .. }) {
                    if self.link_to_last_solve {
                        center_content = center_content.push(self.penalty_selector());
                    }
                    center_content = center_content
                        .push(
                            widget::text(format!(
                                "Ao5: {}",
                                self.session
                                    .last_ao5()
                                    .map_or("--".to_string(), |s| s.to_string()),
                            ))
                            .size(averages_text_font_size),
                        )
                        .push(
                            widget::text(format!(
                                "Ao12: {}",
                                self.session
                                    .last_ao12()
                                    .map_or("--".to_string(), |s| s.to_string()),
                            ))
                            .size(averages_text_font_size),
                        );
                }

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
    ) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
        let label = widget::text(label);

        let style = if self.solve_time.penalty == penalty {
            theme::Button::Primary
        } else {
            theme::Button::Text
        };

        widget::button(label)
            .style(style)
            .padding(8)
            .on_press(Message::PenaltySelected(penalty))
            .into()
    }

    fn penalty_selector(&self) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
        widget::row![
            self.penalty_button("OK", None),
            self.penalty_button("+2", Some(data::Penalty::Plus2)),
            self.penalty_button("DNF", Some(data::Penalty::Dnf)),
        ]
        .spacing(6)
        .align_items(Alignment::Center)
        .into()
    }

    fn sidebar(&self) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
        let times_column = widget::Column::with_children(
            self.session
                .iter()
                .enumerate()
                .rev()
                .map(|se| {
                    widget::row![
                        widget::text(se.1.solve.time)
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .width(Length::FillPortion(1)),
                        widget::text(se.1.ao5.map_or("--".to_string(), |ao5| ao5.to_string()))
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .width(Length::FillPortion(1)),
                        widget::text(se.1.ao12.map_or("--".to_string(), |ao12| ao12.to_string()))
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .width(Length::FillPortion(1)),
                    ]
                    .spacing(8)
                    .into()
                })
                .collect(),
        )
        .spacing(8)
        .padding(12)
        .align_items(Alignment::Start)
        .width(Length::Fixed(300.0));

        widget::scrollable(times_column)
            .vertical_scroll(widget::scrollable::Properties::default())
            .into()
    }

    fn bottombar(&self) -> iced::Element<'_, Message, iced::Renderer<iced::Theme>> {
        let times_row = widget::Row::with_children(
            self.session
                .iter()
                .enumerate()
                .map(|se| widget::text(se.1.solve.time).into())
                .collect(),
        )
        .spacing(8)
        .padding(12)
        .align_items(Alignment::Start);

        widget::scrollable(times_row)
            .horizontal_scroll(widget::scrollable::Properties::default())
            .into()
    }
}
