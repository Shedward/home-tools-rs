use crate::ui::ds;
use shared::rest_client::{endpoint::Endpoint, rest_client};
use std::sync::{Arc, Mutex};

pub type SharedServices = Arc<Mutex<Services>>;

pub struct Services {
    pub theme: ds::theme::Theme,
    pub rest_client: Arc<dyn rest_client::RestClient>,
}

impl Default for Services {
    fn default() -> Self {
        let endpoint = Endpoint {
            host: "home.home".to_string(),
        };
        let rest_client = rest_client::ehttp_client::Client { endpoint };

        Self {
            theme: ds::theme::Theme::dark(),
            rest_client: Arc::new(rest_client),
        }
    }
}

impl Services {
    pub fn create_shared() -> SharedServices {
        Arc::new(Mutex::new(Default::default()))
    }
}

pub trait SharedServicesGetters {
    fn theme(&self) -> ds::theme::Theme;
    fn rest_client(&self) -> Arc<dyn rest_client::RestClient>;
}

impl SharedServicesGetters for SharedServices {
    fn theme(&self) -> ds::theme::Theme {
        self.lock().unwrap().theme
    }

    fn rest_client(&self) -> Arc<dyn rest_client::RestClient> {
        self.lock().unwrap().rest_client.clone()
    }
}
