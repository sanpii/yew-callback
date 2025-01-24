use quote::ToTokens;

#[proc_macro]
pub fn callback(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let clone = syn::parse_macro_input!(input as Callback);
    clone.into_token_stream().into()
}

struct Callback {
    captures: Vec<Capture>,
    body: Closure,
}

impl syn::parse::Parse for Callback {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut captures = Vec::new();

        while let Ok(capture) = Capture::parse(input) {
            captures.push(capture);

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        let body = input.parse::<Closure>()?;

        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
        }

        Ok(Self { captures, body })
    }
}

impl ToTokens for Callback {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let captures = &self.captures;
        let cb = &self.body;

        tokens.extend(quote::quote! {{
            #( #captures )*
            yew::Callback::from(#cb)
        }});
    }
}

struct Capture {
    pub alias: Option<syn::Ident>,
    pub name: syn::Expr,
}

impl syn::parse::Parse for Capture {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let (alias, name) = if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;
            let expr = input.parse::<syn::Expr>()?;
            (Some(name), expr)
        } else {
            let mut segments = syn::punctuated::Punctuated::new();
            segments.push_value(name.into());

            let path = syn::Path {
                leading_colon: None,
                segments,
            };
            let exprpath = syn::ExprPath {
                attrs: Vec::new(),
                qself: None,
                path,
            };

            (None, syn::Expr::Path(exprpath))
        };

        Ok(Self { alias, name })
    }
}

impl ToTokens for Capture {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;

        let token = if let Some(alias) = &self.alias {
            quote::quote! { let #alias = #name.clone(); }
        } else {
            quote::quote! { let #name = #name.clone(); }
        };

        tokens.extend(token);
    }
}

struct Closure {
    cb: syn::ExprClosure,
}

impl syn::parse::Parse for Closure {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self { cb: input.parse()? })
    }
}

impl ToTokens for Closure {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.cb.to_tokens(tokens);
    }
}
