mod inspect_numeric;

pub trait EguiInspect {
    fn inspect(&mut self, ui: &mut egui::Ui);
}

pub trait InspectNumeric {
    fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &str);
    fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, name: &str);
}

