use crate::app::services::*;
use crate::tools::tool::Tool;
use crate::ui::widgets::dot_grid;
use egui::{Ui, Widget};
use shared::api::online;
use shared::rest_client::DefaultResponseHandlers;
use shared::rest_client::*;
use shared::tools::Loading;

pub struct OnlineCountersTool {
    pub shared_services: SharedServices,
    pub rows: u32,
    pub counters_response: rest_client::LoadingValue<Vec<online::OnlineCounter>>,
}

impl OnlineCountersTool {
    pub fn new(shared_services: SharedServices) -> Self {
        Self {
            shared_services,
            rows: 10,
            counters_response: Default::default(),
        }
    }

    fn fetch_online_data(&mut self) {
        let request = online::OnlineCountersRequest::new();
        self.shared_services.rest_client().request(
            request.request(),
            rest_client::ResponseHandler::loading(self.counters_response.clone()),
        );
    }
}

impl Tool for OnlineCountersTool {
    fn id(&self) -> &'static str {
        "online_counters"
    }

    fn title(&self) -> String {
        "Online".to_owned()
    }

    fn icon(&self) -> String {
        egui_phosphor::regular::NETWORK.to_owned()
    }

    fn ui(&mut self, ui: &mut Ui) {
        let theme = self.shared_services.theme();
        dot_grid::DotGrid::new(|pos| dot_grid::Cell {
            fill: theme.colors.plot((pos.x + pos.y % self.rows) as f32 / 20.0),
        })
        .with_size(16, 24)
        .ui(ui);

        let response = self.counters_response.lock().unwrap();
        let status_description = match *response {
            Loading::None => "No data",
            Loading::Loading => "Loading",
            Loading::Loaded(_) => "Loaded",
            Loading::Failed(_) => "Error",
        };
        ui.label(status_description);
    }
}
