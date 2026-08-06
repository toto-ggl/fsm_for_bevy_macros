use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Expr, Lit, Meta, Variant};

pub fn get_component_name(fsm_name: Ident, variant_name: Ident) -> Ident {
    Ident::new(
        &format!("{}{}", fsm_name, variant_name),
        variant_name.span(),
    )
}

pub fn get_fields_quote(variant: &Variant) -> TokenStream {
    if variant.fields.is_empty() {
        return quote! {};
    }

    let fields = variant.fields.iter().map(|_field| quote! { _ });

    quote! { (#(#fields),*) }
}

pub fn get_sparse_set(variant: &Variant) -> TokenStream {
    let is_sparse_set = variant.attrs.iter().any(|attr| {
        if let Meta::NameValue(nv) = &attr.meta {
            match &nv.value {
                Expr::Lit(expr_lit) => match &expr_lit.lit {
                    Lit::Str(lit_str) => lit_str.value().contains("SparseSet"),
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    });

    if is_sparse_set {
        quote! {
            #[component(storage = "SparseSet")]
        }
    } else {
        quote! {}
    }
}
