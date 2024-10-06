use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

#[proc_macro_derive(OrdForPoint)]
pub fn derive_pt_impl(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(token_stream as DeriveInput);
    let ident = input.ident;
    let expanded = quote! {
        impl PartialOrd for #ident {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                std::option::Option::Some(self.cmp(other))
            }
        }
        impl Ord for #ident {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                if self.x() < other.x() {
                    return Ordering::Less;
                }
                
                if self.x() > other.x() {
                    return Ordering::Greater;
                }
                
                self.y().cmp(&other.y())
            }
        }
        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.x() == other.x() && self.y() == other.y()
            }
        }
        impl Eq for #ident {}
        impl Hash for #ident {
            fn hash<H :std::hash::Hasher>(&self, state: &mut H) {
                self.x().hash(state);
                self.y().hash(state);
            }
        }
    };

    TokenStream::from(expanded)
}
