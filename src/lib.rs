mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use utils::{get_component_name, get_fields_quote, get_sparse_set};

#[proc_macro_attribute]
pub fn derive_fsm(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(Component, Copy, Clone, Debug, Default, EnumIter, FSM, PartialEq)]
        #input
    };

    output.into()
}

#[proc_macro_derive(FSM)]
pub fn impl_fsm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Enum(enum_data) = input.data {
        let fsm_name = input.ident;
        let definitions = enum_data.variants.iter().map(|variant| {
            let component_name = get_component_name(fsm_name.clone(), variant.ident.clone());
            let sparse_set = get_sparse_set(variant);

            quote! {
                #[derive(Component)]
                #sparse_set
                pub struct #component_name;
            }
        });
        let insertions = enum_data.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let fields = get_fields_quote(variant);
            let component_name = get_component_name(fsm_name.clone(), variant.ident.clone());

            quote! {
                #fsm_name::#variant_name #fields => commands.entity(entity).insert(#component_name),
            }
        });
        let removals = enum_data.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let fields = get_fields_quote(variant);
            let component_name = get_component_name(fsm_name.clone(), variant.ident.clone());

            quote! {
                #fsm_name::#variant_name #fields => commands.entity(entity).remove::<#component_name>(),
            }
        });
        let implementation = quote! {
            impl FSM for #fsm_name {
                fn insert_marker(state: #fsm_name, commands: &mut Commands, entity: Entity) {
                    match state {
                        #(#insertions)*
                    };
                }
                fn remove_marker(state: #fsm_name, commands: &mut Commands, entity: Entity) {
                    match state {
                        #(#removals)*
                    };
                }
            }
        };
        let expanded = quote! {
            #(#definitions)* #implementation
        };

        return TokenStream::from(expanded);
    }

    panic!("FSM can only be derived for enums!");
}
