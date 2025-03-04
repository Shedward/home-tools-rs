use egui::Widget;

use super::services::{Services, SharedServices};
use crate::tools;
use crate::tools::tool::Tool;
use crate::ui::ds::fonts::Fonts;
use crate::ui::ds::space::Space;
use crate::ui::widgets;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    pub services: SharedServices,
    pub tools: Vec<Rc<RefCell<dyn Tool>>>,

    pub open_tool: Option<Rc<RefCell<dyn Tool>>>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = App {
            services: Services::create_shared(),
            tools: Vec::new(),
            open_tool: None,
        };

        app.add_tool(tools::HomeTool::new());
        app.add_tool(tools::OnlineCountersTool::new(app.services.clone()));
        app.add_tool(tools::AboutTool::new());

        app.open_tool = app.tools.first().cloned();

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("actions")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(Space(2)))
            .exact_width(Space(16).value())
            .show(ctx, |ui| {
                let selected_tool_id = self.open_tool.as_ref().map(|t| t.borrow().id());
                for tool in &self.tools {
                    if widgets::ToolButton::new(tool, Some(tool.borrow().id()) == selected_tool_id)
                        .ui(ui)
                        .clicked()
                    {
                        let open_tool = tool.clone();
                        open_tool.borrow_mut().on_appear();
                        self.open_tool = Some(open_tool);
                    };
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool_cell) = &self.open_tool.as_mut() {
                let mut tool = tool_cell.borrow_mut();
                ui.heading(format!("Home :: {}", tool.title()));
                ui.add_space(Space(2).into());
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

        cc.egui_ctx.set_fonts(Fonts::font_definitions());

        app
    }

    pub fn theme(&self) -> crate::ui::ds::theme::Theme {
        self.services.lock().unwrap().theme
    }

    pub fn add_tool<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.push(Rc::new(RefCell::new(tool)));
    }
}
