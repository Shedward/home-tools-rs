use egui::Widget;

use super::services::{Services, SharedServices};
use crate::tools;
use crate::tools::tool::Tool;
use crate::ui::ds::space::Space;
use crate::ui::widgets;
use std::rc::Rc;

pub struct App {
    pub services: SharedServices,
    pub tools: Vec<Rc<dyn crate::tools::tool::Tool>>,

    pub open_tool: Option<Rc<dyn crate::tools::tool::Tool>>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = App {
            services: Services::create_shared(),
            tools: Vec::new(),
            open_tool: None,
        };

        app.add_tool(tools::OnlineCountersTool {});
        app.add_tool(tools::AboutTool {});

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("actions")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(Space(2)))
            .exact_width(Space(16).value())
            .show(ctx, |ui| {
                for tool in &self.tools {
                    if widgets::tool_button::ToolButton::new(&tool)
                        .ui(ui)
                        .clicked()
                    {
                        self.open_tool = Some(tool.clone());
                    };
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool) = &self.open_tool {
                tool.ui(ui);
            }
        });
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = App::default();

        cc.egui_ctx.set_theme(egui::Theme::Dark);
        cc.egui_ctx.set_style(app.theme().style());

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "phosphor".into(),
            egui_phosphor::Variant::Regular.font_data().into(),
        );

        if let Some(font_keys) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            font_keys.push("phosphor".into());
        }

        app
    }

    pub fn theme(&self) -> crate::ui::ds::theme::Theme {
        self.services.lock().unwrap().theme
    }

    pub fn add_tool<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.push(Rc::new(tool));
    }
}
