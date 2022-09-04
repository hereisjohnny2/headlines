use eframe::{
    egui::{Context, FontData, FontDefinitions, Layout, Separator, Style, TextStyle, Ui, CentralPanel, ScrollArea},
    emath::Align,
    epaint::{Color32, FontFamily, FontId},
};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("url{}", a),
        });

        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn config_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
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
        ]
        .into();

        ctx.set_fonts(font_def);
        ctx.set_style(style);
    }

    pub fn render_news_card(&self, ui: &mut Ui) {
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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Headings");
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.render_news_card(ui);
                })
        });
    }
}

pub struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}
