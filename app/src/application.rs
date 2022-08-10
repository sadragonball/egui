use std::time::Duration;
use eframe::{App, Frame, Storage};
use eframe::emath::Vec2;
use eframe::epaint::Rgba;
use egui::{Context, Visuals};

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    selected_anchor:String,
}

pub struct Application {
    state: State,
    //modules
    volume_renderer: crate::modules::VolumeRenderer,
    //files
    dropped_files: Vec<egui::DroppedFile>,
}

impl Application {
    pub fn  new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            state: State::default(),
            volume_renderer: crate::modules::VolumeRenderer::new(cc),
            dropped_files: Default::default()
        };

        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                app.state = state;
            }
        }

        app
    }
    fn  modules_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let mut vec = vec![(
            "yingtan volume renderer",
            "volume_renderer",
            &mut self.volume_renderer as &mut dyn eframe::App
            )];

        vec.into_iter()
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.state.selected_anchor.is_empty() {
            let selected_anchor = self.modules_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        egui::TopBottomPanel::top("app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            })
        });

        self.show_selected_module(ctx, frame);
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, _storage: &mut dyn Storage) {
        eframe::set_value(_storage, eframe::APP_KEY, &self.state);
    }

    fn clear_color(&self, _visuals: &Visuals) -> Rgba {
        _visuals.window_fill().into()
    }


}

//UI part
impl Application {
    fn show_selected_module(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.to_owned();
        for (_name, _anchor, _module) in self.modules_iter_mut() {
            if _anchor == selected_anchor || ctx.memory().everything_is_visible() {
                _module.update(ctx, frame);
                found_anchor = true;
            }
        }
    }
    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        ui.label("hello, yingtan volume renderer rs");

        ui.separator();

        let mut selected_anchor = self.state.selected_anchor.to_owned();
        for (name, anchor,_module) in self.modules_iter_mut() {
            if ui.selectable_label(selected_anchor == anchor, name).clicked() {
                selected_anchor = anchor.to_owned();
            }
        }
        self.state.selected_anchor = selected_anchor;
        egui::warn_if_debug_build(ui);
    }
}