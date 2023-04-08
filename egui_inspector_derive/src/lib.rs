use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use darling::FromField;


#[derive(Debug, FromField)]
#[darling(attributes(inspect), default)]
struct InspectAttribute {
    widget: Option<String>,
    min: f32,
    max: f32,
    hide: bool,
    speed: f32,
}

impl Default for InspectAttribute {
    fn default() -> Self {
        Self {
            widget: Some("DragValue".to_string()),
            min: 0.0,
            max: 100.0,
            hide: false,
            speed: 1.0,
        }
    }
}

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn egui_inspector_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => &fields.named,
                syn::Fields::Unnamed(_) => panic!("Tuple structs are not supported"),
                syn::Fields::Unit => panic!("Unit structs are not supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let field_code = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = &field_name.to_string();

        let attribute = InspectAttribute::from_field(field).unwrap();
        if attribute.hide { return quote! {} }

        let (min, max) = (attribute.min, attribute.max);
        let speed = attribute.speed;
        let egui_widget = if let Some(widget) = attribute.widget {
            match widget.as_str() {
                "DragValue" => quote! { responses.extend(self.#field_name.inspect_drag_value(ui, #name, #speed)); },
                "Slider" => quote! { responses.extend(self.#field_name.inspect_slider(ui, #min, #max, #name, #speed as f64)); },
                "Color" => quote! { responses.extend(self.#field_name.inspect_color(ui)); },
                _ => panic!("Invalid Widget! Field: {}.{}", name, field_name)
            }
        } else {
            quote! {}
        };

        quote! { #egui_widget }
    });

    let output = quote! {
        impl EguiInspect for #name {
            fn inspect(&mut self, ui: &mut egui::Ui) -> Vec<egui::Response> {
                let mut responses: Vec<egui::Response> = Vec::new();
                #(#field_code)*
                responses
            }
        }
    };
    output.into()
}