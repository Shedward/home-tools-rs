use std::ops::Deref;

use super::online_grid::{self, *};
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
    pub grid: rest_client::LoadingValue<OnlineGrid>,
}

impl OnlineCountersTool {
    pub fn new(shared_services: SharedServices) -> Self {
        Self {
            shared_services,
            rows: 10,
            grid: Default::default(),
        }
    }

    fn fetch_online_data(&mut self) {
        let two_weeks_from_now = chrono::Utc::now() - chrono::Duration::weeks(2);
        let request = online::OnlineCountersRequest::new().from(two_weeks_from_now.naive_local());
        self.shared_services.rest_client().request(
            request.request(),
            rest_client::ResponseHandler::loading_map(self.grid.clone(), OnlineGrid::new),
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

        let grid = self.grid.lock().unwrap();
        dot_grid::DotGrid::new(|pos| dot_grid::Cell {
            fill: match grid.deref() {
                Loading::Loaded(grid) => theme
                    .colors
                    .plot(grid.rel_at(pos.x as usize, pos.y as usize)),
                _ => theme.colors.accessory(),
            },
        })
        .with_size(online_grid::DAYS_COUNT, online_grid::HOURS_COUNT)
        .ui(ui);
    }

    fn on_appear(&mut self) {
        self.fetch_online_data();
    }
}
