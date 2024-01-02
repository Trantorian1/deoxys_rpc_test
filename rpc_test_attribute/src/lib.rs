use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use syn::parse::Parse;

struct Args {
    arg_struct: syn::Ident,
    arg_test: syn::LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arg_struct = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let arg_test = input.parse::<syn::LitStr>()?;

        Ok(Args {
            arg_struct: arg_struct,
            arg_test: arg_test,
        })
    }
}

#[proc_macro_attribute]
pub fn rpc_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    let name = input.sig.ident;
    let arg_struct = args.arg_struct;
    let arg_test = args.arg_test;

    let expression = quote! {
        #[tokio::test]
        async fn #name() {
            let config = rpc_test::test_config::TestConfig::new("./secret.json").unwrap();
            let alchemy = jsonrpsee::http_client::HttpClientBuilder::default().build(config.alchemy).unwrap();

            let path = #arg_test;
            let test_data = rpc_test::test_data::TestData::new(path)
                .with_context(|| format!("Could not retrieve test data from {path}"))
                .unwrap();

            let response: #arg_struct = #arg_struct::call(&alchemy, &test_data.cmd, test_data.arg).await
                .with_context(|| format!("Error waiting for rpc call response in test {path}"))
                .unwrap();

            // FIXME: compare expected to deoxis call
            assert_eq!(1, 0);
        }       
    };

    TokenStream::from(expression)
}