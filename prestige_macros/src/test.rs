use proc_macro::TokenStream;
use syn::ItemFn;

pub fn parse(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let body = &input.block;

    let marker_name = quote::format_ident!("{}_test_marker", name);
    let result = quote::quote! {
        #[test_case]
        static #marker_name: crate::tests::Test = crate::tests::Test {
            func: #name,
            path: concat!(module_path!(), "::", stringify!(#name))
        };

        fn #name() {
            #body
        }
    };

    result.into()
}
