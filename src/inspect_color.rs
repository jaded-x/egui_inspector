use super::InspectColor;

impl InspectColor for [f32; 3] {
    fn inspect_color(&mut self, ui: &mut egui::Ui) -> Vec<egui::Response> {
        vec![egui::color_picker::color_edit_button_rgb(ui, self)]
    }
}