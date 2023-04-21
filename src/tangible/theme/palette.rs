use iced::{color, Color};
use once_cell::sync::Lazy;
use palette::{FromColor, Hsl, Srgb};

#[derive(Debug, Default, Clone, Copy)]
pub struct BackgroundColor {
    pub base: Color,
    pub hover: Color,
    pub active: Color,
}

impl BackgroundColor {
    pub fn new(base: Color) -> Self {
        Self {
            base,
            hover: base,
            active: base,
        }
    }

    pub fn darken_hover(self, amount: f32) -> Self {
        Self {
            hover: darken(self.base, amount),
            ..self
        }
    }

    pub fn lighten_hover(self, amount: f32) -> Self {
        Self {
            hover: lighten(self.base, amount),
            ..self
        }
    }

    pub fn darken_active(self, amount: f32) -> Self {
        Self {
            active: darken(self.base, amount),
            ..self
        }
    }

    pub fn lighten_active(self, amount: f32) -> Self {
        Self {
            active: lighten(self.base, amount),
            ..self
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ColorGroup {
    pub base: Color,
    pub bg: Color,
    pub fg: Color,
    pub base_hover: Color,
    pub bg_hover: Color,
}

impl ColorGroup {
    pub fn darken_hover(self, amount: f32) -> Self {
        Self {
            base_hover: darken(self.base, amount),
            bg_hover: darken(self.bg, amount),
            ..self
        }
    }

    pub fn lighten_hover(self, amount: f32) -> Self {
        Self {
            base_hover: lighten(self.base, amount),
            bg_hover: lighten(self.bg, amount),
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NamedColor {
    Neutral,
    Accent,
    Destructive,
    Success,
    Warning,
    Error,
    Flat,
    Window,
    View,
    Header,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Palette {
    pub neutral: ColorGroup,
    pub accent: ColorGroup,
    pub destructive: ColorGroup,
    pub success: ColorGroup,
    pub warning: ColorGroup,
    pub error: ColorGroup,
    pub flat: ColorGroup,

    pub window: ColorGroup,
    pub view: ColorGroup,
    pub header: ColorGroup,
}

impl Palette {
    pub fn group(&self, name: NamedColor) -> ColorGroup {
        match name {
            NamedColor::Neutral => self.neutral,
            NamedColor::Accent => self.accent,
            NamedColor::Destructive => self.destructive,
            NamedColor::Success => self.success,
            NamedColor::Warning => self.warning,
            NamedColor::Error => self.error,
            NamedColor::Flat => self.flat,
            NamedColor::Window => self.window,
            NamedColor::View => self.view,
            NamedColor::Header => self.header,
        }
    }
}

pub static LIGHT: Lazy<Palette> = Lazy::new(|| Palette {
    neutral: ColorGroup {
        base: color!(0x000000, 0.1),
        bg: color!(0x000000, 0.1),
        fg: color!(0x000000, 0.8),
        bg_hover: color!(0x000000, 0.2),
        ..Default::default()
    },
    accent: ColorGroup {
        base: color!(0x1c71d8),
        bg: color!(0x3584e4),
        fg: color!(0xffffff),
        ..Default::default()
    }
    .lighten_hover(0.1),
    destructive: ColorGroup {
        base: color!(0xc01c28),
        bg: color!(0xe01b24),
        fg: color!(0xffffff),
        ..Default::default()
    }
    .lighten_hover(0.1),
    success: ColorGroup {
        base: color!(0x1b8553),
        bg: color!(0x2ec27e),
        fg: color!(0xffffff),
        ..Default::default()
    }
    .darken_hover(0.1),
    warning: ColorGroup {
        base: color!(0x9c6e03),
        bg: color!(0xe5a50a),
        fg: color!(0x000000, 0.8),
        ..Default::default()
    }
    .darken_hover(0.1),
    error: ColorGroup {
        base: color!(0xc01c28),
        bg: color!(0xe01b24),
        fg: color!(0xffffff),
        ..Default::default()
    }
    .lighten_hover(0.1),
    flat: ColorGroup {
        bg: color!(0x000000, 0.0),
        fg: color!(0x000000, 0.8),
        bg_hover: color!(0x000000, 0.1),
        ..Default::default()
    },
    window: ColorGroup {
        base: color!(0xfafafa),
        bg: color!(0xfafafa),
        fg: color!(0x000000, 0.8),
        ..Default::default()
    },
    view: ColorGroup {
        base: color!(0xffffff),
        bg: color!(0xffffff),
        fg: color!(0x000000, 0.8),
        ..Default::default()
    },
    header: ColorGroup {
        base: color!(0xffffff),
        bg: color!(0xffffff),
        fg: color!(0x000000, 0.8),
        ..Default::default()
    },
});

fn darken(color: Color, amount: f32) -> Color {
    let mut hsl = Hsl::from_color(Srgb::from(color));

    hsl.lightness = (hsl.lightness - amount).max(0.0);

    Srgb::from_color(hsl).into()
}

fn lighten(color: Color, amount: f32) -> Color {
    let mut hsl = Hsl::from_color(Srgb::from(color));

    hsl.lightness = (hsl.lightness + amount).min(1.0);

    Srgb::from_color(hsl).into()
}
