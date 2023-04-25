use crate::tangible;

pub struct Preferences {
    pub theme: tangible::Theme,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            theme: tangible::Theme::Light,
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Self::new()
    }
}
