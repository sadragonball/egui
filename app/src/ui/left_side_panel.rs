use egui::Widget;
use crate::ui::RunMode;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LeftSidePanel {
    pub open: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    run_mode: RunMode,

    #[cfg_attr(feature = "serde", serde(skip))]
    repaint_after_seconds: f32,

    #[cfg_attr(feature = "serde", serde(skip))]
    frame_history: crate::ui::FrameHistory,
}

impl Default for LeftSidePanel {
    fn default() -> Self {
        Self {
            open: false,
            run_mode: Default::default(),
            repaint_after_seconds: 1.,
            frame_history: Default::default(),
        }
    }
}

impl LeftSidePanel {
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input().time, frame.info().cpu_usage);

        match self.run_mode {
            RunMode::Continuous => {
                ctx.request_repaint();
            }
            RunMode::Reactive => {
                ctx.request_repaint_after(
                    std::time::Duration::from_secs_f32(
                        self.repaint_after_seconds
                    ));
            }
        }
    }


    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::trace!(ui);
        self.integration_ui(ui, frame);

        ui.separator();

        {
            let mut debug_on_hover = ui.ctx().debug_on_hover();
            ui.checkbox(&mut debug_on_hover, "🔧 Debug on hover")
              .on_hover_text("Show structure of the ui when you hover with the mouse");
            ui.ctx().set_debug_on_hover(debug_on_hover);
        }
    }

    fn integration_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.;
            ui.label("egui running inside ");
            ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
            ui.label(".");
        });

        ui.horizontal(|ui| {
            let mut fullscreen = frame.info().window_info.fullscreen;
            ui.checkbox(&mut fullscreen, "🗖 Fullscreen")
              .on_hover_text("Fullscreen the window");
            frame.set_fullscreen(fullscreen);
        });
    }
}


