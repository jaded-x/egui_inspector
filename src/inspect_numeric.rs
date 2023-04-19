use super::InspectNumeric;

macro_rules! impl_inspect_numeric {
    ($($t:ty),+) => {
        $(
            impl InspectNumeric for $t {
                fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &str, speed: f32, min: f32, max: f32) -> Vec<egui::Response> {
                    let mut responses: Vec<egui::Response> = Vec::new();
                    ui.horizontal(|ui| {
                        ui.label(name);
                        responses.push(ui.add(egui::DragValue::new(self)
                            .speed(speed)
                            .clamp_range(min..=max)
                        ));
                    });

                    responses
                }

                fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, name: &str, speed: f64) -> Vec<egui::Response> {
                    let mut responses: Vec<egui::Response> = Vec::new();
                    ui.horizontal(|ui| {
                        responses.push(ui.add(egui::Slider::new(self, (min as $t)..=(max as $t))
                            .drag_value_speed(speed)
                        ));
                        ui.label(name);
                    });

                    responses
                }
            }
        )+
    }
}

impl_inspect_numeric!(
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);


macro_rules! impl_inspect_generic {
    ($c:ident::$vec:ident $fields:tt, $($t:ty),+) => {
        $(
            impl_inspect_generic!(@fields $c::$vec $fields, $t);
        )+
    };

    (@fields $c:ident::$vec:ident($($field:ident),*), $t:ty) => {
        impl InspectNumeric for $c::$vec<$t> {
            fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &str, speed: f32, min: f32, max: f32) -> Vec<egui::Response> {
                let mut responses: Vec<egui::Response> = Vec::new();
                
                ui.label(name);
                ui.horizontal(|ui| {
                    $( responses.extend(self.$field.inspect_drag_value(ui, stringify!($field), speed, min, max)); )*
                });

                responses
            }

            fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, name: &str, speed: f64) -> Vec<egui::Response> {
                let mut responses: Vec<egui::Response> = Vec::new();
                
                ui.vertical(|ui| {
                    ui.label(name);
                    $( responses.extend(self.$field.inspect_slider(ui, min, max, stringify!($field), speed)); )*
                });

                responses
            }
        }
    }
}

impl_inspect_generic!(
    cg::Vector3(x, y, z), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);

impl_inspect_generic!(
    cg::Vector4(x, y, z, w), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);