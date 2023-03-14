use super::InspectNumeric;

macro_rules! impl_inspect_numeric {
    ($($t:ty),+) => {
        $(
            impl InspectNumeric for $t {
                fn inspect_drag_value(&mut self, ui: &mut egui::Ui, _name: &str) {
                    ui.add(egui::DragValue::new(self).speed(0.01));
                }

                fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, _name: &str) {
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
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


macro_rules! impl_inspect_vector {
    ($c:ident::$vec:ident $fields:tt, $($t:ty),+) => {
        $(
            impl_inspect_vector!(@fields $c::$vec $fields, $t);
        )+
    };

    (@fields $c:ident::$vec:ident($($field:ident),*), $t:ty) => {
        impl InspectNumeric for $c::$vec<$t> {
            fn inspect_drag_value(&mut self, ui: &mut egui::Ui, name: &str) {
                ui.horizontal(|ui| {
                    $( self.$field.inspect_drag_value(ui, stringify!($field)); )*
                    ui.label(name);
                });
            }

            fn inspect_slider(&mut self, ui: &mut egui::Ui, min: f32, max: f32, name: &str) {
                ui.vertical(|ui| {
                    $( self.$field.inspect_slider(ui, min, max, stringify!($field)); )+
                    ui.label(name);
                });
            }
        }
    }
}

impl_inspect_vector!(
    cg::Vector3(x, y, z), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);

impl_inspect_vector!(
    cg::Vector4(x, y, z, w), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);