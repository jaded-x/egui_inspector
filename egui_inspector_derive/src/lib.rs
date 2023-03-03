use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Meta, Lit, MetaNameValue};
use darling::{FromField, FromMeta};


#[derive(Debug, FromField)]
#[darling(attributes(inspect))]
struct InspectAttribute {
    widget: Option<String>
}

#[proc_macro_derive(EguiInspect, attributes(inspect))]
pub fn egui_inspector_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => &fields.named,
                Fields::Unnamed(_) => panic!("Tuple structs are not supported"),
                Fields::Unit => panic!("Unit structs are not supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let field_code = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let attribute = InspectAttribute::from_field(field).unwrap_or(InspectAttribute {widget: None});
        println!("{:?}", attribute.widget);
        let widget_type = if let Some(widget) = attribute.widget {
            match widget.as_str() {
                "DragValue" => quote! { self.#field_name.inspect_drag_value(ui); },
                _ => quote! {}
            }
        } else {
            quote! {}
        };

        quote! {
            #widget_type
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