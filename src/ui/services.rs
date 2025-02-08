use std::sync::{Arc, Mutex};

use super::ds;

pub type SharedServices = Arc<Mutex<Services>>;

pub struct Services {
    pub theme: ds::Theme,
}

impl Services {
    pub fn create_shared() -> SharedServices {
        let services = Self {
            theme: ds::Theme::light(),
        };

        Arc::new(Mutex::new(services))
    }
}
