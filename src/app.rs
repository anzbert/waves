use crate::engine::Engine;
use egui::{
    plot::{HLine, Line, Plot, PlotPoints, VLine},
    Vec2,
};
use std::{
    f64::consts::{PI, TAU},
    sync::{Arc, Mutex},
};

pub struct WavesApp {
    phase: f64,
    freq: f64,
    frame_size: usize,
    engine: Engine,
    // tx: Sender<EngineFn>,
    // func: Arc<Mutex<EngineFn>>,
}

impl WavesApp {
    /// Called once before the first frame.
    ///
    /// This is also where you can customized the look at feel of egui using
    /// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // let (tx, rx) = mpsc::channel::<EngineFn>();

        Self {
            phase: 0.,
            freq: 440.,
            frame_size: 2048,
            engine: Engine::new(),
            // tx,
        }
    }
}

impl eframe::App for WavesApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.add(
                egui::Slider::new(&mut self.phase, 0.0..=PI)
                    .custom_formatter(|val, _range| format!("{:.2}", val / PI))
                    .text("Phase"),
            );

            ui.add(
                egui::Slider::new(&mut self.freq, 0.0..=PI)
                    .custom_formatter(|val, _range| format!("{:.2}", val / PI))
                    .text("Freq"),
            );

            egui::TopBottomPanel::bottom("audio_info_box").show(ctx, |ui| {
                ui.label(&self.engine.cpal.info);
            });
        });

        // The central panel the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot: PlotPoints = (0..self.frame_size)
                .map(|x| {
                    let sample = x as f64 / self.frame_size as f64;

                    let y = (self.phase + sample * TAU).sin();
                    // let y_cosine_synth = (sample * TAU).cos() * (1. - (sample * 2.5 * TAU).sin());

                    [x as f64, y]
                })
                .collect();

            let phase: f64 = self.phase.clone();
            let freq: f64 = self.freq.clone();
            let fs: usize = self.frame_size.clone();

            let func =
                move |x| ((phase + x / fs as f64 * TAU * (freq - 440.).max(0.)) as f64).sin();

            // let plot = PlotPoints::from_explicit_callback(
            //     self.func.func,
            //     0.0..self.frame_size as f64,
            //     self.frame_size,
            // );

            let iter = (0..2048).map(|x| [x as f64, (func)(x as f64)]);

            let plot = PlotPoints::from_iter(iter);

            let line = Line::new(plot).width(3.);

            Plot::new("plot")
                .allow_drag(false)
                .allow_boxed_zoom(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .set_margin_fraction(Vec2::new(0.1, 0.1))
                .include_x(0.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                    plot_ui.vline(VLine::new(self.frame_size as f64));
                    plot_ui.vline(VLine::new(0.));
                    plot_ui.hline(HLine::new(1.));
                    plot_ui.hline(HLine::new(-1.));
                });
        });
    }
}
