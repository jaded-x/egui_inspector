use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field};
use darling::FromField;


#[derive(Debug, FromField)]
#[darling(attributes(inspect), default)]
struct InspectAttribute {
    widget: Option<String>,
    min: f32,
    max: f32,
}

impl Default for InspectAttribute {
    fn default() -> Self {
        Self {
            widget: Some("DragValue".to_string()),
            min: 0.0,
            max: 100.0
        }
    }
}

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn egui_inspector_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let fields: Vec<Field> = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => fields.named.iter().filter(|f| {
                    f.attrs.iter().any(|a| {
                        if let Ok(meta) = a.parse_meta() {
                            meta.path().is_ident("inspect")
                        } else { false }
                    })
                }).cloned().collect(),
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
        let (min, max) = (attribute.min, attribute.max);
        let egui_widget = if let Some(widget) = attribute.widget {
            match widget.as_str() {
                "DragValue" => quote! { self.#field_name.inspect_drag_value(ui, #name); },
                "Slider" => quote! { self.#field_name.inspect_slider(ui, #min, #max); },
                _ => panic!("Widget not valid! Field: {}.{}", name, field_name)
            }
        } else {
            quote! {}
        };
        quote! {
            #egui_widget
        }
    });

    let output = quote! {
        impl EguiInspect for #name {
            fn inspect(&mut self, ui: &mut egui::Ui) {
                #(#field_code)*
            }
        }
    };
    output.into()
}