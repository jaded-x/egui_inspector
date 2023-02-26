use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn egui_inspector_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => fields,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let mut egui_tokens = Vec::new();
    for field in fields.named.iter() {
        if let syn::Type::Path(type_path) = &field.ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "f32" {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_span = field.ident.as_ref().unwrap().span();
                    egui_tokens.push(quote_spanned! {field_span=>
                        ui.add(egui::DragValue::new(&mut self.#field_name).speed(0.02));
                    });
                }
            }
        }
    }
    let result = quote! {
        impl EguiInspect for #name {
            fn inspect(&mut self, ui: &mut egui::Ui) {
                #(#egui_tokens)*
            }
        }
    };
    result.into()
}