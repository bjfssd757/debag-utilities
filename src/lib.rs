extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn, FnArg, Pat, PatIdent, Ident};

#[proc_macro]
pub fn dbg_here(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as Ident);
    let name = input_parsed.to_string();

    let expanded: TokenStream;

    if name == "" {
        expanded = quote! {
            {
                use colored::{self, Colorize};

                let message = format!("\n[{}:{}] in {}\n", file!(), line!(), module_path!()).blue();
                eprintln!("{}", message);
                #input_parsed
            }
        }.into();
    } else {
        expanded = quote! {
            {
                use colored::{self, Colorize};

                let message = format!("\n[{}:{}] in {}: '{}' = {:?}\n",
                    file!(),
                    line!(),
                    module_path!(),
                    #name,
                    &#input_parsed
                ).blue();
                eprintln!("{}", message);
                #input_parsed
            }
        }.into();
    }

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_generics = &input_fn.sig.generics;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_asyncness = &input_fn.sig.asyncness;
    let fn_constness = &input_fn.sig.constness;
    let fn_unsafety = &input_fn.sig.unsafety;
    let fn_abi = &input_fn.sig.abi;
    
    let arg_names: Vec<_> = fn_inputs.iter().enumerate().filter_map(|(i, arg)| {
        match arg {
            FnArg::Typed(pat_typed) => {
                if let Pat::Ident(PatIdent { ident, .. }) = &*pat_typed.pat {
                    Some(ident.clone())
                } else {
                    let ident = format_ident!("arg_{}", i);
                    Some(ident)
                }
            }
            FnArg::Receiver(_) => {
                let ident = format_ident!("self");
                Some(ident)
            }
        }
    }).collect();
    
    let log_args = arg_names.iter().map(|name| {
        quote! {
            let message_param = format!("    Param '{}' = {:?}\n", stringify!(#name), #name).blue();
            println!("{}", message_param);
        }
    });
    
    let expanded = quote! {
        use colored::{self, Colorize};

        #fn_vis #fn_constness #fn_asyncness #fn_unsafety #fn_abi fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            let message_enter = format!("\n--> Entering function: '{}'", stringify!(#fn_name)).blue();
            println!("{}", message_enter);
            #(#log_args)*
            
            let result = {
                #fn_block
            };
            
            let message_exit = format!("\n<-- Exiting function: '{}' with result: {:?}\n", stringify!(#fn_name), result).blue();
            println!("{}", message_exit);
            result
        }
    };
    
    TokenStream::from(expanded)
}