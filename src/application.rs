use std::time::{Duration, Instant};

pub struct KubiaTimer {
    duration: Duration,
    state: State
}

pub enum State {
    Idle,
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

    type Message = ();

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                duration: Duration::default(),
                state: State::Idle,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kubia Timer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}
