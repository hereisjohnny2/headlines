mod headlines_config;
mod news_card_data;

use headlines_config::HeadlinesConfig;
use news_card_data::NewsCardData;

use eframe::{
    egui::{
        menu, CentralPanel, Context, FontData, FontDefinitions, Layout, RichText, ScrollArea,
        Separator, Style, TextStyle, TopBottomPanel, Ui, Visuals,
    },
    emath::Align,
    epaint::{Color32, FontFamily, FontId},
};

use confy;

const PADDING: f32 = 5.0;

const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const LIGHT_BLUE: Color32 = Color32::from_rgb(0, 255, 255);
const BLUE: Color32 = Color32::from_rgb(0, 102, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
    config: HeadlinesConfig,
}

impl Headlines {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let iter = (0..20).map(|a| {
            NewsCardData::new(
                format!("title{}", a),
                format!("description{}", a),
                format!("http://url/{}", a),
            )
        });

        let config: HeadlinesConfig = confy::load("headlines_config").unwrap_or_default();

        Headlines {
            articles: Vec::from_iter(iter),
            config,
        }
    }

    pub fn config_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();

        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../../../MesloLGS_NF_Regular.ttf")),
        );

        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());

        let mut style = Style::default();

        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(35.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
            (
                TextStyle::Small,
                FontId::new(16.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(20.0, FontFamily::Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);
        ctx.set_fonts(font_def);
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.label(RichText::new("📓").heading());
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("❌").clicked() {
                        frame.close();
                    }

                    if ui.button("🔄").clicked() {
                        todo!();
                    }

                    if ui
                        .button(if self.config.dark_mode() {
                            "🌙"
                        } else {
                            "🌞"
                        })
                        .clicked()
                    {
                        self.config.toggle_dark_mode();
                    }
                });
            });
            ui.add_space(10.);
        });
    }

    fn render_news_card(&self, ui: &mut Ui) {
        for article in &self.articles {
            // Add title
            ui.add_space(PADDING);
            let title = format!("▶ {}", article.title());
            ui.colored_label(if self.config.dark_mode() { WHITE } else { BLACK }, title);

            // Add description
            ui.add_space(PADDING);
            ui.label(RichText::new(article.desc()).small());

            // Add URL
            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = if self.config.dark_mode() {
                LIGHT_BLUE
            } else {
                BLUE
            };
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.hyperlink_to("read more ⤴", &article.url());
            });

            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

impl eframe::App for Headlines {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        if self.config.dark_mode() {
            ctx.set_visuals(Visuals::dark())
        } else {
            ctx.set_visuals(Visuals::light())
        }

        self.render_top_panel(ctx, frame);
        render_footer(ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    self.render_news_card(ui);
                });
        });
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Headlines");
    });

    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.));
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.label(RichText::new("API source: newsapi.org").small());
            ui.hyperlink_to(
                RichText::new("Made with egui").small(),
                "https://github.com/emilk/egui",
            );
            ui.hyperlink_to(
                RichText::new("hereisjohnny2/headlines").small(),
                "https://github.com/hereisjohnny2/headlines",
            );
            ui.add_space(10.);
        });
    });
}
