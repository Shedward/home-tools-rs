use std::sync::{Arc, Mutex};

use crate::ui::ds;

pub type SharedServices = Arc<Mutex<Services>>;

pub struct Services {
    pub theme: ds::theme::Theme,
}

impl Default for Services {
    fn default() -> Self {
        Self {
            theme: ds::theme::Theme::dark(),
        }
    }
}

impl Services {
    pub fn create_shared() -> SharedServices {
        Arc::new(Mutex::new(Default::default()))
    }
}
