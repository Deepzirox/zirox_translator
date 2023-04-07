use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;
use quote::quote_spanned;
use quote::ToTokens;

use syn::Expr;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token;

// getting custom magic keys
use super::custom_keys;


/*
                ALLOCATOR GRAMMAR - STRUCT TO PARSE VARIABLE DECLARATIONS TO AST TREE
*/
// I want to create an Expr parser for example:
// let var_name = "hello world";
pub struct AllocatorGrammar {
    let_sym : custom_keys::set,
    var_name: syn::Ident,
    equal_sym: token::Eq,
    value: Expr,
    span: Span,
}

impl Parse for AllocatorGrammar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let start_span = input.cursor().span();
        let let_sym = input.parse()?;
        let var_name = input.parse()?;
        let equal_sym = input.parse()?;
        let value: Expr = input.parse()?;
        let end_span = input.cursor().span();
        let span = start_span.join(end_span).unwrap_or(start_span);
        

        Ok(Self {
            let_sym,
            var_name,
            equal_sym,
            value,
            span,

        })
    }
}

impl ToTokens for AllocatorGrammar {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        
        let let_sym = &self.let_sym;
        let var_name = &self.var_name;
        let equal_sym = &self.equal_sym;
        let value = &self.value;
        let span = self.span;
 

        tokens.extend(quote_spanned! { span =>
           #let_sym
           #var_name
           #equal_sym
           #value
        });
    }
}

impl AllocatorGrammar {
    pub fn translate(input: &str) -> Result<TokenStream, String> {
        let tokens = syn::parse_str::<AllocatorGrammar>(input);
        if let Err(e) = tokens {
            return Err(e.to_string().clone());
        }

        let tokens = tokens.unwrap();
        Ok(tokens.into_token_stream())
    }
}

/*
                                ALLOCATOR GRAMMAR - END BLOCK
*/



/*
                                Expr GRAMMAR - Start BLOCK
*/

// Handling expr it's easy, it is pre-defined
pub struct ExprGrammar;

impl ExprGrammar {
    pub fn translate(input: &str) -> Result<TokenStream, String> {
        let tokens = syn::parse_str::<Expr>(input);
        if let Err(e) = tokens {
            return Err(e.to_string().clone());
        }

        let tokens = tokens.unwrap();
        Ok(tokens.into_token_stream())
    }
}

/*
                                Expr GRAMMAR - end BLOCK
*/


/*
                                BULTIN Utilities GRAMMAR - Start BLOCK
*/

pub struct StdoutWriteGrammar {
    id: custom_keys::stdout,
    handler: custom_keys::write,
    value: Ident
}

impl Parse for StdoutWriteGrammar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id = input.parse()?;
        let handler = input.parse()?;
        let value = input.parse()?;

        Ok(Self {
            id,
            handler,
            value
        })
    }
}

impl ToTokens for StdoutWriteGrammar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.id;
        let handler = &self.handler;
        let value = &self.value;

        tokens.extend(quote!{
            #id
            #handler
            #value
         });
    }
}

impl StdoutWriteGrammar {
    pub fn translate(input: &str) -> Result<TokenStream, String> {
        let tokens = syn::parse_str::<StdoutWriteGrammar>(input);
        if let Err(e) = tokens {
            return Err(e.to_string().clone());
        }

        let tokens = tokens.unwrap();
        Ok(tokens.into_token_stream())
    }
}
