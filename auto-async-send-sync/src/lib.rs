use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemTrait, ReturnType, TraitItem, parse_macro_input};

#[proc_macro_attribute]
pub fn auto_send_sync(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(input as ItemTrait);

    let vis = &input_trait.vis;
    let ident = &input_trait.ident;
    let generics = &input_trait.generics;
    let supertraits = &input_trait.supertraits;
    let where_clause = &generics.where_clause;

    let mut items = Vec::new();

    for item in &input_trait.items {
        match item {
            TraitItem::Fn(func) => {
                let mut func = func.clone();

                if func.sig.asyncness.is_some() {
                    let attrs = &func.attrs;
                    let sig = &mut func.sig;
                    let block = func.default.as_ref();

                    // Remove `async` from signature
                    sig.asyncness = None;

                    let output_ty = match &sig.output {
                        ReturnType::Type(_, ty) => quote! { #ty },
                        ReturnType::Default => quote! { () },
                    };

                    // Replace return type with future so we can attach the send bound
                    sig.output = syn::parse_quote! {
                        -> impl std::future::Future<Output = #output_ty> + Send
                    };

                    // If there's a default implementation (i.e., method body), wrap it in `async move` so it returns a future
                    let new_func = block.map_or_else(
                        || {
                            quote! {
                                #(#attrs)*
                                #sig;
                            }
                        },
                        |body| {
                            let stmts = &body.stmts;
                            quote! {
                                #(#attrs)*
                                #sig {
                                    async move {
                                        #(#stmts)*
                                    }
                                }
                            }
                        },
                    );

                    items.push(new_func);
                } else {
                    // Leave non-async methods untouched
                    items.push(func.to_token_stream());
                }
            }
            other => {
                items.push(other.to_token_stream());
            }
        }
    }

    let expanded = quote! {
        #vis trait #ident #generics: #supertraits #where_clause {
            #(#items)*
        }
    };

    TokenStream::from(expanded)
}
