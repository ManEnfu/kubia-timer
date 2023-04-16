use iced::{
    keyboard::{self, KeyCode},
    subscription, time, widget, Color, Command, Event, Length, Subscription,
};
use std::time::{Duration, Instant};

pub struct KubiaTimer {
    duration: Duration,
    last_pressed: Instant,
    state: State,
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

        let duration_label = widget::text(format!(
            "{:0>2}:{:0>2}.{:0>2}",
            seconds / 60,
            seconds % 60,
            self.duration.subsec_millis() / 10,
        ))
        .style(match self.state {
            State::Idle { pressed: true } => Color::new(1.0, 0.0, 0.0, 1.0),
            State::Finished => Color::new(1.0, 0.0, 0.0, 1.0),
            State::Ready => Color::new(0.0, 1.0, 0.0, 1.0),
            _ => Color::new(0.0, 0.0, 0.0, 1.0),
        })
        .size(40);

        widget::container(duration_label)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
