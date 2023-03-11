pub trait EguiInspect {
    fn inspect(&mut self, ui: &mut egui::Ui);
}

pub trait InspectNumeric {
    fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &'static str);
    fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32);
}

macro_rules! impl_inspect_numeric {
    ($($t:ty),+) => {
        $(
            impl InspectNumeric for $t {
                fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &'static str) {
                    ui.add(egui::DragValue::new(self).speed(0.01));
                }

                fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32) {
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                }
            }
        )+
    }
}

macro_rules! impl_inspect_vector {
    ($t:ty) => {
        impl InspectNumeric for $t {
            fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &'static str) {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.x).speed(0.01));
                    ui.add(egui::DragValue::new(&mut self.y).speed(0.01));
                    ui.add(egui::DragValue::new(&mut self.z).speed(0.01));
                    ui.label(name);
                });
                println!("{}", stringify!($t));
            }

            fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32) {
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.x, min..=max));
                    ui.add(egui::Slider::new(&mut self.y, min..=max));
                    ui.add(egui::Slider::new(&mut self.z, min..=max));
                });
            }
        }
    };
}

impl_inspect_numeric!(f32, f64);
impl_inspect_numeric!(i8, i16, i32, i64, isize);
impl_inspect_numeric!(u8, u16, u32, u64, usize);

impl_inspect_vector!(cg::Vector3<f32>);

// impl InspectNumeric for cg::Vector3<f32> {
//     fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &'static str) {
//         ui.horizontal(|ui| {
//             ui.label(name);
//             ui.add(egui::DragValue::new(&mut self.x));
//             ui.add(egui::DragValue::new(&mut self.y));
//             ui.add(egui::DragValue::new(&mut self.z));
//         });
//     }

//     fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32) {
//         ui.vertical(|ui| {
//             ui.add(egui::Slider::new(&mut self.x, min..=max));
//             ui.add(egui::Slider::new(&mut self.y, min..=max));
//             ui.add(egui::Slider::new(&mut self.z, min..=max));
//         });
//     }
// }