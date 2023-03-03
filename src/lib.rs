pub trait EguiInspect {
    fn inspect(&mut self, ui: &mut egui::Ui);
}

pub trait InspectNumeric {
    fn inspect_drag_value(&mut self, ui: &mut egui::Ui);
    fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32);
}

macro_rules! impl_inspect_float {
    ($($t:ty),+) => {
        $(
            impl InspectNumeric for $t {
                fn inspect_drag_value(&mut self, ui: &mut egui::Ui) {
                    ui.add(egui::DragValue::new(self).speed(0.01));
                }

                fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32) {
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                }
            }
        )+
    }
}

impl_inspect_float!(f32, f64);
impl_inspect_float!(i64);