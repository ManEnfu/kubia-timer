use iced::{color, Color};
use once_cell::sync::Lazy;
use palette::{FromColor, Hsl, Srgb};

#[derive(Debug, Default, Clone, Copy)]
pub struct BackgroundColor {
    pub base: Color,
    pub hover: Color,
    pub strong: Color,
}

impl BackgroundColor {
    pub fn new(base: Color) -> Self {
        Self {
            base,
            hover: base,
            strong: base,
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

    pub fn darken_strong(self, amount: f32) -> Self {
        Self {
            strong: darken(self.base, amount),
            ..self
        }
    }

    pub fn lighten_strong(self, amount: f32) -> Self {
        Self {
            strong: lighten(self.base, amount),
            ..self
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ColorGroup {
    pub base: Color,
    pub bg: BackgroundColor,
    pub fg: Color,
    pub border: Color,
    // pub base_hover: Color,
    // pub bg_hover: Color,
}

// impl ColorGroup {
//     pub fn darken_hover(self, amount: f32) -> Self {
//         Self {
//             base_hover: darken(self.base, amount),
//             bg_hover: darken(self.bg, amount),
//             ..self
//         }
//     }

//     pub fn lighten_hover(self, amount: f32) -> Self {
//         Self {
//             base_hover: lighten(self.base, amount),
//             bg_hover: lighten(self.bg, amount),
//             ..self
//         }
//     }
// }

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

    pub view: ColorGroup,
    pub card: ColorGroup,
    pub header: ColorGroup,

    pub selector: ColorGroup,
    pub selector_active: ColorGroup,
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
            NamedColor::Window => self.view,
            NamedColor::View => self.card,
            NamedColor::Header => self.header,
        }
    }
}

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

pub static PALETTE_LIGHT: Lazy<Palette> = Lazy::new(|| Palette {
    neutral: ColorGroup {
        base: color!(0x000000, 0.3),
        bg: BackgroundColor {
            base: color!(0x000000, 0.3),
            hover: color!(0x000000, 0.45),
            strong: color!(0x000000, 0.60),
        },
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    accent: ColorGroup {
        base: color!(0x1c71d8),
        bg: BackgroundColor::new(color!(0x3584e4))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    destructive: ColorGroup {
        base: color!(0xc01c28),
        bg: BackgroundColor::new(color!(0xe01b24))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    success: ColorGroup {
        base: color!(0x1b8553),
        bg: BackgroundColor::new(color!(0x2ec27e))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    warning: ColorGroup {
        base: color!(0x9c6e03),
        bg: BackgroundColor::new(color!(0xe5a50a))
            .darken_hover(0.1)
            .darken_strong(0.2),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    error: ColorGroup {
        base: color!(0xc01c28),
        bg: BackgroundColor::new(color!(0xe01b24))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    flat: ColorGroup {
        base: color!(0x000000, 0.0),
        bg: BackgroundColor {
            base: color!(0x000000, 0.0),
            hover: color!(0x000000, 0.2),
            strong: color!(0x000000, 0.4),
        },
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    view: ColorGroup {
        base: color!(0xfafafa),
        bg: BackgroundColor::new(color!(0xfafafa)).darken_strong(0.15),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    card: ColorGroup {
        base: color!(0xffffff),
        bg: BackgroundColor::new(color!(0xffffff))
            .darken_hover(0.2)
            .darken_strong(0.3),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    header: ColorGroup {
        base: color!(0xffffff),
        bg: BackgroundColor::new(color!(0xffffff)).darken_strong(0.15),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    selector: ColorGroup {
        base: color!(0x000000, 0.3),
        bg: BackgroundColor {
            base: color!(0x000000, 0.3),
            hover: color!(0x000000, 0.45),
            strong: color!(0x000000, 0.60),
        },
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    selector_active: ColorGroup {
        base: color!(0xfafafa),
        bg: BackgroundColor::new(color!(0xfafafa)).darken_strong(0.15),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
});

pub static PALETTE_DARK: Lazy<Palette> = Lazy::new(|| Palette {
    neutral: ColorGroup {
        base: color!(0xffffff, 0.05),
        bg: BackgroundColor {
            base: color!(0xffffff, 0.02),
            hover: color!(0xffffff, 0.04),
            strong: color!(0xffffff, 0.06),
        },
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    accent: ColorGroup {
        base: color!(0x78aeed),
        bg: BackgroundColor::new(color!(0x3584e4))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    destructive: ColorGroup {
        base: color!(0xff7b63),
        bg: BackgroundColor::new(color!(0xc01c28))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    success: ColorGroup {
        base: color!(0x8ff0a4),
        bg: BackgroundColor::new(color!(0x26a269))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    warning: ColorGroup {
        base: color!(0xf8e45c),
        bg: BackgroundColor::new(color!(0xcd9309))
            .darken_hover(0.1)
            .darken_strong(0.2),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    error: ColorGroup {
        base: color!(0xff7b63),
        bg: BackgroundColor::new(color!(0xc01c28))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    flat: ColorGroup {
        base: color!(0x000000, 0.0),
        bg: BackgroundColor {
            base: color!(0xffffff, 0.0),
            hover: color!(0xffffff, 0.02),
            strong: color!(0xffffff, 0.04),
        },
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    view: ColorGroup {
        base: color!(0x242424),
        bg: BackgroundColor::new(color!(0x242424)).lighten_strong(0.05),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    card: ColorGroup {
        base: color!(0x303030),
        bg: BackgroundColor::new(color!(0x303030))
            .lighten_hover(0.10)
            .lighten_strong(0.15),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    header: ColorGroup {
        base: color!(0x303030),
        bg: BackgroundColor::new(color!(0x303030)).lighten_strong(0.05),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    selector: ColorGroup {
        base: color!(0xffffff, 0.05),
        bg: BackgroundColor {
            base: color!(0xffffff, 0.02),
            hover: color!(0xffffff, 0.04),
            strong: color!(0xffffff, 0.06),
        },
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    selector_active: ColorGroup {
        base: color!(0x242424),
        bg: BackgroundColor::new(color!(0x242424)).lighten_strong(0.05),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
});

pub static PALETTE_TANGIBLE: Lazy<Palette> = Lazy::new(|| Palette {
    neutral: ColorGroup {
        base: color!(0xffffff, 0.05),
        bg: BackgroundColor {
            base: color!(0xffffff, 0.02),
            hover: color!(0xffffff, 0.04),
            strong: color!(0xffffff, 0.06),
        },
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    accent: ColorGroup {
        base: color!(0x78aeed),
        bg: BackgroundColor::new(color!(0x3584e4))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    destructive: ColorGroup {
        base: color!(0xff7b63),
        bg: BackgroundColor::new(color!(0xc01c28))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    success: ColorGroup {
        base: color!(0x8ff0a4),
        bg: BackgroundColor::new(color!(0x26a269))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    warning: ColorGroup {
        base: color!(0xf8e45c),
        bg: BackgroundColor::new(color!(0xcd9309))
            .darken_hover(0.1)
            .darken_strong(0.2),
        fg: color!(0x202020),
        border: color!(0x202020, 0.2),
    },
    error: ColorGroup {
        base: color!(0xff7b63),
        bg: BackgroundColor::new(color!(0xc01c28))
            .lighten_hover(0.1)
            .darken_strong(0.1),
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    flat: ColorGroup {
        base: color!(0x000000, 0.0),
        bg: BackgroundColor {
            base: color!(0xffffff, 0.0),
            hover: color!(0xffffff, 0.02),
            strong: color!(0xffffff, 0.04),
        },
        fg: color!(0xffffff),
        border: color!(0xffffff, 0.08),
    },
    view: ColorGroup {
        base: color!(0x101010),
        bg: BackgroundColor {
            base: color!(0x101010),
            hover: color!(0x202020),
            strong: color!(0x080808),
        },
        fg: color!(0xffffff),
        border: color!(0x323232),
    },
    card: ColorGroup {
        base: color!(0x202020),
        bg: BackgroundColor {
            base: color!(0x202020),
            hover: color!(0x303030),
            strong: color!(0x424242),
        },
        fg: color!(0xffffff),
        border: color!(0x000000),
    },
    header: ColorGroup {
        base: color!(0x202020),
        bg: BackgroundColor::new(color!(0x202020)).lighten_strong(0.05),
        fg: color!(0xffffff),
        border: color!(0x323232),
    },
    selector: ColorGroup {
        base: color!(0x202020),
        bg: BackgroundColor {
            base: color!(0x202020),
            hover: color!(0x303030),
            strong: color!(0x424242),
        },
        fg: color!(0xffffff),
        border: color!(0x000000),
    },
    selector_active: ColorGroup {
        base: color!(0x04323f),
        bg: BackgroundColor {
            base: color!(0x15424f),
            hover: color!(0x245462),
            strong: color!(0x245462),
        },
        fg: color!(0xffffff),
        border: color!(0x000000),
    },
});
