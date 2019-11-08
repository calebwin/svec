extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Block, Expr, ExprIf, Token};

// look at an if statement
//
// modify the then branch to push its contents to Vec
// modify the else branch to push its contents to Vec if else is block
fn modify_if(mut if_item: ExprIf) -> ExprIf {
    // modify then branch
    let new_then_branch = if_item.then_branch.clone();
    if_item.then_branch = syn::parse::<Block>(
        quote! {
            {
                temp.push(#new_then_branch);
            }
        }
        .into(),
    )
    .unwrap();

    // modify else branch
    // reurse if this is an else if
    // stop recursion if this is a block
    if let Some(else_branch) = if_item.else_branch.clone() {
        let new_else_branch = else_branch.clone();

        // handle base case of else block
        if let Expr::Block(else_branch_block) = *new_else_branch.1.clone() {
            if_item.else_branch = Some((
                new_else_branch.0,
                Box::new(
                    syn::parse::<Expr>(
                        quote! {
                            {
                                temp.push(#else_branch_block);
                            }
                        }
                        .into(),
                    )
                    .unwrap(),
                ),
            ))
        }

        // handle else if with recursion
        if let Expr::If(if_branch_block) = *new_else_branch.1 {
            if_item.else_branch = Some((
                new_else_branch.0,
                Box::new(Expr::If(modify_if(if_branch_block))),
            ));
        }
    }

    if_item
}

#[proc_macro_hack]
pub fn svec(input: TokenStream) -> TokenStream {
    // parse into list
    let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
    let list = parser.parse(input).unwrap();

    // iterate over list
    // generate insertions
    let mut insertions = vec![];
    for item in list {
        match item {
            Expr::If(mut if_item) => {
                if_item = modify_if(if_item);

                insertions.push(quote! {
                    #if_item
                });
            }
            Expr::ForLoop(mut for_item) => {
                let old_for_item = for_item.clone();
                let old_body = old_for_item.body;

                for_item.body = syn::parse::<Block>(
                    quote! {
                        {
                            temp.push(#old_body);
                        }
                    }
                    .into(),
                )
                .unwrap();

                insertions.push(quote! {
                    #for_item
                });
            }
            _ => insertions.push(quote! {
                temp.push(#item);
            }),
        }
    }

    (quote! {
        {
            let mut temp = Vec::new();
            #(#insertions)*
            temp
        }
    })
    .into()
}
