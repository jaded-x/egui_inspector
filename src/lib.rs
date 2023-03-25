mod inspect_numeric;

pub trait EguiInspect {
    fn inspect(&mut self, ui: &mut egui::Ui) -> Vec<egui::Response>;
}

pub trait InspectNumeric {
    fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &str, speed: f32) -> Vec<egui::Response>;
    fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, name: &str, speed: f64) -> Vec<egui::Response>;
}

