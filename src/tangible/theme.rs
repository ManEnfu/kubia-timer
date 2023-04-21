use std::borrow::Borrow;

pub use self::palette::{ColorGroup, NamedColor, Palette};

use iced::{overlay, widget, Background, Color, Vector};

mod palette;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Tangible,
}

impl Theme {
    pub fn palette(&self) -> &Palette {
        &palette::LIGHT
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => "Light",
                Self::Dark => "Dark",
                Self::Tangible => "Tangible",
            }
        )
    }
}

#[derive(Default)]
pub enum Application {
    #[default]
    Default,

    Custom(Box<dyn iced::application::StyleSheet<Style = Theme>>),
}

impl iced::application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        let palette = self.palette();

        match style {
            Application::Default => iced::application::Appearance {
                background_color: palette.window.bg.base,
                text_color: palette.window.fg,
            },
            Application::Custom(custom) => custom.appearance(self),
        }
    }
}

/// The style for an application.
#[derive(Default, Clone)]
pub enum Button {
    #[default]
    Default,
    Suggested,
    Destructive,
    Flat,
    ColorGroup(ColorGroup),
    // Text,
}

impl From<ColorGroup> for Button {
    fn from(value: ColorGroup) -> Self {
        Self::ColorGroup(value)
    }
}

impl widget::button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> widget::button::Appearance {
        let palette = self.palette();

        let cg = match style {
            Button::Default => palette.neutral,
            Button::Suggested => palette.accent,
            Button::Destructive => palette.destructive,
            Button::Flat => palette.flat,
            Button::ColorGroup(cg) => *cg,
        };

        widget::button::Appearance {
            shadow_offset: Vector::default(),
            background: Some(cg.bg.base.into()),
            border_radius: 6.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: cg.fg,
        }
    }

    fn hovered(&self, style: &Self::Style) -> widget::button::Appearance {
        let palette = self.palette();

        let cg = match style {
            Button::Default => palette.neutral,
            Button::Suggested => palette.accent,
            Button::Destructive => palette.destructive,
            Button::Flat => palette.flat,
            Button::ColorGroup(cg) => *cg,
        };

        widget::button::Appearance {
            background: Some(cg.bg.hover.into()),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> widget::button::Appearance {
        let palette = self.palette();

        let cg = match style {
            Button::Default => palette.neutral,
            Button::Suggested => palette.accent,
            Button::Destructive => palette.destructive,
            Button::Flat => palette.flat,
            Button::ColorGroup(cg) => *cg,
        };

        widget::button::Appearance {
            background: Some(cg.bg.strong.into()),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> widget::button::Appearance {
        let active = self.active(style);

        widget::button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

#[derive(Default, Clone)]
pub enum Checkbox {
    #[default]
    Default,
    Success,
    Destructive,
}

impl widget::checkbox::StyleSheet for Theme {
    type Style = Checkbox;

    fn active(&self, style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        let palette = self.palette();

        let cg = if is_checked {
            match style {
                Checkbox::Default => palette.accent,
                Checkbox::Success => palette.success,
                Checkbox::Destructive => palette.destructive,
            }
        } else {
            palette.neutral
        };

        widget::checkbox::Appearance {
            background: cg.bg.base.into(),
            icon_color: cg.fg,
            border_radius: 6.0,
            text_color: None,
            border_width: 1.0,
            border_color: cg.bg.strong,
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> widget::checkbox::Appearance {
        let palette = self.palette();

        let cg = if is_checked {
            match style {
                Checkbox::Default => palette.accent,
                Checkbox::Success => palette.success,
                Checkbox::Destructive => palette.destructive,
            }
        } else {
            palette.neutral
        };

        widget::checkbox::Appearance {
            background: cg.bg.hover.into(),
            icon_color: cg.fg,
            border_radius: 6.0,
            text_color: None,
            border_width: 1.0,
            border_color: cg.bg.strong,
        }
    }
}

/// The style of a container.
#[derive(Default, Clone)]
pub enum Container {
    #[default]
    Transparent,
    Solid(NamedColor),
}

impl From<NamedColor> for Container {
    fn from(value: NamedColor) -> Self {
        Self::Solid(value)
    }
}

impl widget::container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> widget::container::Appearance {
        let palette = self.palette();

        match style {
            Container::Transparent => Default::default(),
            Container::Solid(name) => {
                let cg = palette.group(*name);
                widget::container::Appearance {
                    text_color: Some(cg.fg),
                    background: Some(cg.bg.base.into()),
                    border_radius: 6.0,
                    ..Default::default()
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub enum Slider {
    #[default]
    Default,
}

impl widget::slider::StyleSheet for Theme {
    type Style = Slider;

    fn active(&self, style: &Self::Style) -> widget::vertical_slider::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style) -> widget::vertical_slider::Appearance {
        todo!()
    }

    fn dragging(&self, style: &Self::Style) -> widget::vertical_slider::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum Menu {
    #[default]
    Default,
}

impl overlay::menu::StyleSheet for Theme {
    type Style = Menu;

    fn appearance(&self, style: &Self::Style) -> overlay::menu::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum PickList {
    #[default]
    Default,
}

impl widget::pick_list::StyleSheet for Theme {
    type Style = PickList;

    fn active(
        &self,
        style: &<Self as widget::pick_list::StyleSheet>::Style,
    ) -> widget::pick_list::Appearance {
        todo!()
    }

    fn hovered(
        &self,
        style: &<Self as widget::pick_list::StyleSheet>::Style,
    ) -> widget::pick_list::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum Radio {
    #[default]
    Default,
}

impl widget::radio::StyleSheet for Theme {
    type Style = Radio;

    fn active(&self, style: &Self::Style, is_selected: bool) -> widget::radio::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> widget::radio::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum Toggler {
    #[default]
    Default,
}

impl widget::toggler::StyleSheet for Theme {
    type Style = Toggler;

    fn active(&self, style: &Self::Style, is_active: bool) -> widget::toggler::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> widget::toggler::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum PaneGrid {
    #[default]
    Default,
}

impl widget::pane_grid::StyleSheet for Theme {
    type Style = PaneGrid;

    fn picked_split(&self, style: &Self::Style) -> Option<widget::pane_grid::Line> {
        todo!()
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<widget::pane_grid::Line> {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum ProgressBar {
    #[default]
    Accent,
    Success,
    Warning,
    Error,
}

impl widget::progress_bar::StyleSheet for Theme {
    type Style = ProgressBar;

    fn appearance(&self, style: &Self::Style) -> widget::progress_bar::Appearance {
        todo!()
    }
}

#[derive(Default, Clone)]
pub enum Rule {
    #[default]
    Default,
}

impl widget::rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, _style: &Self::Style) -> widget::rule::Appearance {
        let palette = self.palette();

        widget::rule::Appearance {
            color: palette.view.bg.strong,
            width: 1,
            radius: 0.0,
            fill_mode: widget::rule::FillMode::Full,
        }
    }
}

#[derive(Default, Clone)]
pub enum Svg {
    #[default]
    Default,
}

#[derive(Default, Clone)]
pub enum Scrollable {
    #[default]
    Default,
}

impl widget::scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> widget::scrollable::Scrollbar {
        let palette = self.palette();

        widget::scrollable::Scrollbar {
            background: palette.window.bg.base.into(),
            border_radius: 6.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: widget::scrollable::Scroller {
                color: palette.neutral.bg.base,
                border_color: Color::TRANSPARENT,
                border_width: 0.0,
                border_radius: 2.0,
            },
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> widget::scrollable::Scrollbar {
        if is_mouse_over_scrollbar {
            let palette = self.palette();

            widget::scrollable::Scrollbar {
                background: palette.window.bg.base.into(),
                border_radius: 6.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: widget::scrollable::Scroller {
                    color: palette.neutral.bg.hover,
                    border_color: Color::TRANSPARENT,
                    border_width: 0.0,
                    border_radius: 2.0,
                },
            }
        } else {
            self.active(style)
        }
    }

    fn dragging(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        self.hovered(style, true)
    }

    fn active_horizontal(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        self.active(style)
    }

    fn hovered_horizontal(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> widget::scrollable::Scrollbar {
        self.hovered(style, is_mouse_over_scrollbar)
    }

    fn dragging_horizontal(&self, style: &Self::Style) -> widget::scrollable::Scrollbar {
        self.hovered_horizontal(style, true)
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Color(Color),
}

impl From<Color> for Text {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl widget::text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> widget::text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => widget::text::Appearance { color: Some(c) },
        }
    }
}

#[derive(Default, Clone)]
pub enum TextInput {
    #[default]
    Default,
}

impl widget::text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> widget::text_input::Appearance {
        todo!()
    }

    fn focused(&self, style: &Self::Style) -> widget::text_input::Appearance {
        todo!()
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        todo!()
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        todo!()
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        todo!()
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        todo!()
    }

    fn disabled(&self, style: &Self::Style) -> widget::text_input::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style) -> widget::text_input::Appearance {
        self.focused(style)
    }
}
