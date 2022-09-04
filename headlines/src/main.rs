use eframe::{
    egui::{self, FontDefinitions, Layout, Separator},
    emath::Align,
    epaint::{Color32, Vec2},
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 960.));

    eframe::run_native(
        "Headings",
        native_options,
        Box::new(|cc| {
            let app = Headlines::new(cc);
            app.config_fonts(&cc.egui_ctx);
            Box::new(app)
        }),
    )
}

#[derive(Default)]
struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("url{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    fn config_fonts(&self, ctx: &egui::Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            egui::FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        font_def
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        let mut style = egui::Style::default();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(35.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();

        ctx.set_fonts(font_def);
        ctx.set_style(style);
    }

    fn render_news_card(&self, ui: &mut egui::Ui) {
        for article in &self.articles {
            // Add title
            ui.add_space(PADDING);
            let title = format!("▶ {}", article.title);
            ui.colored_label(WHITE, title);

            // Add description
            ui.add_space(PADDING);
            ui.label(&article.desc);

            // Add URL
            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.hyperlink_to("read more ⤴", &article.url);
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

impl eframe::App for Headlines {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Headings");
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.render_news_card(ui);
                })
        });
    }
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}
