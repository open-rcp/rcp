use egui::{Context, Ui, Color32, Stroke, Rounding, Vec2};

// UI constants
pub const PRIMARY_COLOR: Color32 = Color32::from_rgb(0, 102, 204);   // #0066cc
pub const SECONDARY_COLOR: Color32 = Color32::from_rgb(220, 220, 220); // #dcdcdc
pub const BACKGROUND_COLOR: Color32 = Color32::from_rgb(245, 245, 245); // #f5f5f5
pub const TEXT_COLOR: Color32 = Color32::from_rgb(33, 33, 33);       // #212121
pub const ERROR_COLOR: Color32 = Color32::from_rgb(244, 67, 54);     // #f44336

// UI components
pub fn styled_button(ui: &mut Ui, text: &str) -> egui::Response {
    let btn_stroke = Stroke::new(1.0, SECONDARY_COLOR);
    let bg_fill = PRIMARY_COLOR;
    let text_color = Color32::WHITE;
    
    ui.add(
        egui::Button::new(egui::RichText::new(text).color(text_color))
            .rounding(Rounding::same(4.0))
            .min_size(Vec2::new(120.0, 32.0))
            .fill(bg_fill)
            .stroke(btn_stroke)
    )
}

pub fn styled_window(ctx: &Context, title: &str, content: impl FnOnce(&mut Ui)) {
    egui::Window::new(title)
        .frame(
            egui::Frame::none()
                .fill(BACKGROUND_COLOR)
                .rounding(Rounding::same(8.0))
                .stroke(Stroke::new(1.0, SECONDARY_COLOR))
                .shadow(egui::epaint::Shadow::small_dark())
        )
        .show(ctx, content);
}

pub fn app_card(ui: &mut Ui, name: &str, description: &str, last_used: Option<&str>) -> egui::Response {
    let frame = egui::Frame::none()
        .fill(Color32::WHITE)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0, SECONDARY_COLOR))
        .inner_margin(egui::Margin::same(12.0))
        .outer_margin(egui::Margin::same(0.0));
        
    frame.show(ui, |ui| {
        ui.heading(name);
        ui.label(description);
        
        if let Some(last_used) = last_used {
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(format!("Last used: {}", last_used))
                    .size(11.0)
                    .color(Color32::GRAY)
            );
        }
        
        ui.add_space(8.0);
        styled_button(ui, "Launch")
    }).response
}
