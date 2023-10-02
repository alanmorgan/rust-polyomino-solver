#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(OrdForPointT)]
pub fn derive_pointt_impl(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
    };

    TokenStream::from(expanded)
}
