//! This crate provide a way that can use
//! [Stream](https://docs.rs/futures-util/0.3/futures_util/stream/trait.Stream.html) with for loop
//! like Iterator.

extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, ExprForLoop, Pat, Result, Stmt, Token};

struct AsyncFor {
    pat: Pat,
    stream: Expr,
    loop_body: Vec<Stmt>,
}

impl Parse for AsyncFor {
    fn parse(input: ParseStream) -> Result<Self> {
        let _async: Token![async] = input.parse()?;
        let for_loop: ExprForLoop = input.parse()?;

        Ok(Self {
            pat: for_loop.pat,
            stream: *for_loop.expr,
            loop_body: for_loop.body.stmts,
        })
    }
}

#[proc_macro]
/// This macro can let you use async
/// [Stream](https://docs.rs/futures-util/0.3/futures_util/stream/trait.Stream.html) with for loop
/// like Iterator.
///
/// # Example
///```rust
/// use async_for::async_for;
/// use futures_util::{stream, StreamExt};
///
/// futures_executor::block_on(async {
///     let mut result = vec![];
///
///  async_for! {
///      async for n in stream::iter(vec![1, 2, 3]) {
///          result.push(n);
///      }
///  }
///
///  assert_eq!(result, vec![1, 2, 3]);
/// })
/// ```
pub fn async_for(item: TokenStream) -> TokenStream {
    let AsyncFor {
        pat,
        stream,
        loop_body,
    } = parse_macro_input!(item as AsyncFor);

    let expand = quote! {
        let stream = #stream;

        pin_utils::pin_mut!(stream);

        while let Some(#pat) = stream.next().await {
            #(#loop_body)*
        }
    };

    TokenStream::from(expand)
}
