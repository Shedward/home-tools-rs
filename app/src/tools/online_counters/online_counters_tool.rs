use std::ops::Deref;

use super::online_grid::{self, *};
use crate::app::services::*;
use crate::tools::tool::Tool;
use crate::ui::widgets::dot_grid;
use egui::{ComboBox, RichText, Ui, Widget};
use shared::api;
use shared::rest_client::DefaultResponseHandlers;
use shared::rest_client::*;
use shared::tools::*;

pub struct OnlineCountersTool {
    pub shared_services: SharedServices,

    pub grid: rest_client::LoadingValue<OnlineGrid>,
    pub devices_list: rest_client::LoadingValue<Vec<api::OnlineDevice>>,
    pub selected_device: Option<api::OnlineDevice>,
}

impl OnlineCountersTool {
    pub fn new(shared_services: SharedServices) -> Self {
        Self {
            shared_services,
            grid: Default::default(),
            devices_list: Default::default(),
            selected_device: None,
        }
    }

    fn fetch_online_data(&mut self) {
        let two_weeks_from_now = chrono::Utc::now() - chrono::Duration::weeks(2);
        let mac = self.selected_device.as_ref().map(|d| d.mac.clone());
        let request = api::GetOnlineCounters::new()
            .from(two_weeks_from_now.naive_local())
            .mac(mac);

        self.shared_services.rest_client().api_request(
            &request,
            rest_client::ResponseHandler::loading_map(self.grid.clone(), OnlineGrid::new),
        );
    }

    fn fetch_devices_data(&mut self) {
        let request = api::GetOnlineDevices::new();
        self.shared_services.rest_client().api_request(
            &request,
            rest_client::ResponseHandler::loading(self.devices_list.clone()),
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

        ComboBox::from_id_salt("selected_device")
            .selected_text(self.selected_device.display_name())
            .show_ui(ui, |ui| {
                let devices = { self.devices_list.lock().unwrap().deref().clone() };

                match &devices {
                    Loading::Loaded(devices) => {
                        for device in devices {
                            let display_name = device.display_name();
                            let selection = Some(device.clone());
                            if ui
                                .selectable_label(selection == self.selected_device, display_name)
                                .clicked()
                            {
                                self.selected_device = selection;
                                self.fetch_online_data();
                            }
                        }
                    }
                    Loading::Loading => {
                        ui.label("Loading...");
                    }
                    Loading::Failed(_) => {
                        ui.label("Failed");
                    }
                    Loading::None => {}
                }
            });

        let grid = self.grid.lock().unwrap();
        let shimmer_color = theme.colors.shimmer(ui);

        dot_grid::DotGrid::new(|pos| dot_grid::Cell {
            fill: match grid.deref() {
                Loading::Loaded(grid) => theme
                    .colors
                    .plot(grid.rel_at(pos.x as usize, pos.y as usize)),
                Loading::Loading => shimmer_color,
                Loading::None => shimmer_color,
                Loading::Failed(_) => theme.colors.accessory(),
            },
        })
        .with_size(online_grid::DAYS_COUNT, online_grid::HOURS_COUNT)
        .ui(ui);

        if let Loading::Failed(grid_error) = grid.deref() {
            ui.label(RichText::new(grid_error.to_string()).color(theme.colors.attention));
        }
    }

    fn on_appear(&mut self) {
        self.fetch_online_data();
        self.fetch_devices_data();
    }
}
