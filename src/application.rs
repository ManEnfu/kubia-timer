use iced::{
    keyboard::{self, KeyCode},
    subscription, time, widget, Alignment, Color, Command, Event, Length, Subscription,
};
use std::time::{Duration, Instant, SystemTime};

use crate::data::{Session, Solve, SolveTime};

pub struct KubiaTimer {
    duration: Duration,
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
pub enum Message {
    Press,
    Release,
    Tick(Instant),
}

impl iced::Application for KubiaTimer {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                duration: Duration::default(),
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
            Message::Press => {
                self.last_pressed = Instant::now();
            }
            _ => {}
        }

        match &mut self.state {
            State::Idle { pressed } => match message {
                Message::Press => {
                    *pressed = true;
                }
                Message::Release => {
                    *pressed = false;
                }
                Message::Tick(now) => {
                    if *pressed && now - self.last_pressed > PRESS_START_INTERVAL {
                        self.state = State::Ready;
                        self.duration = Duration::new(0, 0);
                    }
                }
            },

            State::Ready => match message {
                Message::Release => {
                    self.state = State::Timing {
                        last_tick: Instant::now(),
                    };
                }
                _ => {}
            },

            State::Timing { last_tick } => match message {
                Message::Press => {
                    log::info!("Solve: {}", SolveTime::new(self.duration, None));
                    self.session.add_solve(Solve {
                        time: SolveTime::new(self.duration, None),
                        timestamp: SystemTime::now(),
                        scramble: "".to_string(),
                    });
                    self.state = State::Finished;
                }
                Message::Tick(now) => {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                _ => {}
            },

            State::Finished => match message {
                Message::Release => self.state = State::Idle { pressed: false },
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
                    } => Some(Message::Press),
                    keyboard::Event::KeyReleased {
                        key_code: KeyCode::Space,
                        ..
                    } => Some(Message::Release),
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
        let seconds = self.duration.as_secs();
        let duration_label_color = match self.state {
            State::Idle { pressed: true } => Color::new(1.0, 0.0, 0.0, 1.0),
            State::Finished => Color::new(1.0, 0.0, 0.0, 1.0),
            State::Ready => Color::new(0.0, 1.0, 0.0, 1.0),
            _ => Color::new(0.0, 0.0, 0.0, 1.0),
        };

        let duration_text = widget::row![
            widget::text(if seconds >= 60 {
                format!("{:0>1}:{:0>2}", seconds / 60, seconds % 60,)
            } else {
                format!("{:0>1}", seconds,)
            })
            .style(duration_label_color)
            .size(120),
            widget::text(format!(".{:0>2}", self.duration.subsec_millis() / 10,))
                .style(duration_label_color)
                .size(90),
        ]
        .align_items(Alignment::End);

        let mut center_content = widget::column![duration_text,]
            .spacing(12)
            .align_items(Alignment::Center);

        if !matches!(self.state, State::Ready | State::Timing { .. }) {
            center_content = center_content
                .push(
                    widget::text(format!(
                        "Ao5: {}",
                        self.session
                            .last_ao5()
                            .map_or("--".to_string(), |s| s.to_string()),
                    ))
                    .size(30),
                )
                .push(
                    widget::text(format!(
                        "Ao12: {}",
                        self.session
                            .last_ao12()
                            .map_or("--".to_string(), |s| s.to_string()),
                    ))
                    .size(30),
                );
        }

        widget::container(center_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
