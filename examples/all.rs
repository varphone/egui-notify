use eframe::egui::FontDefinitions;
use eframe::{
    egui::{Context, Slider, Window},
    App, Frame, NativeOptions,
};
use egui::{Color32, FontId, Shadow, Style, Vec2, Visuals};
use egui_notify::{Anchor, Toast, Toasts};
use std::time::Duration;

struct ExampleApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    show_progress_bar: bool,
    expires: bool,
    duration: f32,
    font_size: f32,
    dark: bool,
    custom_level_string: String,
    custom_level_color: egui::Color32,
    shadow: bool,
    anchor: Anchor,
    reverse: bool,
    show_inside: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.caption);
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.anchor, Anchor::TopLeft, "TopLeft");
                ui.selectable_value(&mut self.anchor, Anchor::TopRight, "TopRight");
                ui.selectable_value(&mut self.anchor, Anchor::BottomLeft, "BottomLeft");
                ui.selectable_value(&mut self.anchor, Anchor::BottomRight, "BottomRight");
            });
            ui.checkbox(&mut self.reverse, "Reverse");
            ui.checkbox(&mut self.expires, "Expires");
            ui.checkbox(&mut self.closable, "Closable");
            ui.checkbox(&mut self.show_progress_bar, "ShowProgressBar");
            ui.checkbox(&mut self.shadow, "Shadow").clicked().then(|| {
                self.toasts = if self.shadow {
                    Toasts::default().with_shadow(Shadow {
                        offset: Default::default(),
                        blur: 30.0,
                        spread: 5.0,
                        color: Color32::from_black_alpha(70),
                    })
                } else {
                    Toasts::default()
                };
            });
            ui.checkbox(&mut self.show_inside, "ShowInside");
            if !(self.expires || self.closable) {
                ui.label("Warning; toasts will have to be closed programatically");
            }
            ui.add_enabled_ui(self.expires, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Duration (in s)");
                    ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Font size");
                    ui.add(Slider::new(&mut self.font_size, 8.0..=20.0));
                });
            });
            ui.horizontal(|ui| {
                ui.label("Custom level:");
                ui.text_edit_singleline(&mut self.custom_level_string);
                ui.color_edit_button_srgba(&mut self.custom_level_color);
            });

            let customize_toast = |t: &mut Toast| {
                let duration = if self.expires {
                    Some(Duration::from_millis((1000. * self.duration) as u64))
                } else {
                    None
                };
                t.closable(self.closable)
                    .duration(duration)
                    .show_progress_bar(self.show_progress_bar)
                    .font(FontId::proportional(self.font_size));
            };

            ui.horizontal_wrapped(|ui| {
                if ui.button("Success").clicked() {
                    customize_toast(self.toasts.success(self.caption.clone()));
                }

                if ui.button("Info").clicked() {
                    customize_toast(self.toasts.info(self.caption.clone()));
                }

                if ui.button("Warning").clicked() {
                    customize_toast(self.toasts.warning(self.caption.clone()));
                }

                if ui.button("Error").clicked() {
                    customize_toast(self.toasts.error(self.caption.clone()));
                }

                if ui.button("Basic").clicked() {
                    customize_toast(self.toasts.basic(self.caption.clone()));
                }

                if ui.button("Custom").clicked() {
                    customize_toast(self.toasts.custom(
                        self.caption.clone(),
                        self.custom_level_string.clone(),
                        self.custom_level_color,
                    ));
                }

                if ui
                    .button("Phosphor")
                    .on_hover_text("This toast uses egui-phosphor icons")
                    .clicked()
                {
                    customize_toast(self.toasts.custom(
                        self.caption.clone(),
                        egui_phosphor::regular::FAN.to_owned(),
                        self.custom_level_color,
                    ));
                }

                if ui
                    .button("Unique")
                    .on_hover_text("This toast uses unique id to update")
                    .clicked()
                {
                    let msg: String = format!(
                        "Total frames rendered: {}\n{}",
                        ctx.frame_nr(),
                        self.caption.clone()
                    );
                    let toast = Toast::info(msg).with_id("frame_nr");
                    customize_toast(self.toasts.add(toast));
                }

                if ui.button("Error (Red text)").clicked() {
                    customize_toast(
                        self.toasts
                            .error(self.caption.clone())
                            .set_text_color(Some(Color32::RED)),
                    );
                }
            });

            ui.separator();

            if ui.button("Dismiss all toasts").clicked() {
                self.toasts.dismiss_all_toasts();
            }
            if ui.button("Dismiss latest toast").clicked() {
                self.toasts.dismiss_latest_toast();
            }
            if ui.button("Dismiss oldest toast").clicked() {
                self.toasts.dismiss_oldest_toast();
            }

            ui.separator();

            if ui.radio(self.dark, "Toggle dark theme").clicked() {
                self.dark = !self.dark;

                let mut style = ctx.style().as_ref().clone();
                if self.dark {
                    style.visuals = Visuals::dark();
                } else {
                    style.visuals = Visuals::light();
                }
                ctx.set_style(style);
            }
        });

        self.toasts.set_anchor(self.anchor);
        self.toasts.set_reverse(self.reverse);

        if self.show_inside {
            let color = ctx.style().visuals.extreme_bg_color;
            Window::new("Toasts")
                .frame(egui::Frame::window(&ctx.style()).fill(color))
                .show(ctx, |ui| {
                    ui.heading("Toasts Container");
                    ui.separator();
                    ui.label("Some widgets here.");
                    let (_id, _rect) = ui.allocate_space(Vec2::new(320.0, 240.0));
                    self.toasts.show_inside(ui, true);
                });
        } else {
            self.toasts.show(ctx);
        }
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "example",
        NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style::default());

            let mut font_def = FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut font_def, egui_phosphor::Variant::Regular);
            for data in font_def.font_data.values_mut() {
                data.tweak.scale = 1.25;
            }
            cc.egui_ctx.set_fonts(font_def);

            Ok(Box::new(ExampleApp {
                caption: r#"Hello! It's a multiline caption
Next line
Another one
And another one"#
                    .into(),
                toasts: Toasts::default(),
                closable: true,
                expires: true,
                show_progress_bar: true,
                duration: 3.5,
                dark: true,
                font_size: 16.,
                custom_level_string: "$".into(),
                custom_level_color: egui::Color32::GREEN,
                shadow: true,
                anchor: Anchor::TopRight,
                reverse: false,
                show_inside: false,
            }))
        }),
    )
}
