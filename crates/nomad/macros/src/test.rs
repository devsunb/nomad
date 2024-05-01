use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Dollar};
use syn::{
    parse_quote,
    Expr,
    FnArg,
    Ident,
    ItemFn,
    LitInt,
    Pat,
    Result,
    ReturnType,
};
use syn_derive_args::Parse;

#[inline]
pub fn test(args: Args, item: ItemFn) -> Result<TokenStream> {
    let test_attrs = item
        .attrs
        .iter()
        .map(ToTokens::into_token_stream)
        .collect::<proc_macro2::TokenStream>();

    let test_name = &item.sig.ident;

    let inputs = TestInputs::new(&args, &item)?;

    let test = Test::new(&inputs, &item)?;

    let maybe_terminator = test
        .terminator()
        .map(ToTokens::into_token_stream)
        .unwrap_or_else(|| quote! {});

    let output = test.output();

    let body = test.body();

    Ok(quote! {
        #[::nomad::nvim::test(
            nvim_oxi = ::nomad::nvim,
            library_path = ::nomad::tests::library_path(env!("CARGO_CRATE_NAME")),
        )]
        #test_attrs
        fn #test_name(#maybe_terminator) #output {
            #body
        }
    })
}

struct TestInputs {
    inputs: Punctuated<Expr, Comma>,
    definitions: TokenStream,
}

impl TestInputs {
    /// Returns the `let {input} = ..;` definition for every test input.
    fn definitions(&self) -> &TokenStream {
        &self.definitions
    }

    /// Returns the comma-separated list of inputs to call the test function
    /// with.
    fn inputs(&self) -> &Punctuated<Expr, Comma> {
        &self.inputs
    }

    fn new(args: &Args, item: &ItemFn) -> Result<Self> {
        let seed = Seed::new(args, &item.sig.inputs)?;

        let define_seed = seed.definition();
        let mut print_seed = quote!();
        let mut define_generator = quote!();

        let mut inputs = Punctuated::<Expr, Comma>::new();

        if !seed.is_none() {
            print_seed = {
                let seed_name = seed.name();
                quote! { println!("seed: {}", #seed_name); }
            };

            define_generator = {
                let seed_name = seed.name();
                quote! {
                    let mut generator =
                        ::nomad::tests::Generator::new(#seed_name);
                }
            };

            inputs.push(parse_quote! { &mut generator });
        }

        let definitions = quote! {
            #define_seed
            #print_seed
            #define_generator
        };

        Ok(Self { inputs, definitions })
    }
}

enum Test {
    Sync(SyncTest),
    Async(AsyncTest),
}

impl Test {
    fn body(&self) -> &TokenStream {
        match self {
            Self::Sync(test) => test.body(),
            Self::Async(test) => test.body(),
        }
    }

    fn output(&self) -> &ReturnType {
        match self {
            Self::Sync(test) => test.output(),
            Self::Async(test) => test.output(),
        }
    }

    fn new(inputs: &TestInputs, item: &ItemFn) -> Result<Self> {
        if item.sig.asyncness.is_some() {
            Ok(Self::Async(AsyncTest::new(inputs, item)))
        } else {
            Ok(Self::Sync(SyncTest::new(inputs, item)))
        }
    }

    /// Returns the `terminator: Terminator` argument to be used in the
    /// signature of the test function if the test is async, or `None`
    /// otherwise.
    fn terminator(&self) -> Option<FnArg> {
        match self {
            Self::Sync(_) => None,
            Self::Async(test) => Some(test.terminator()),
        }
    }
}

struct SyncTest {
    body: TokenStream,
    output: ReturnType,
}

impl SyncTest {
    fn body(&self) -> &TokenStream {
        &self.body
    }

    fn output(&self) -> &ReturnType {
        &self.output
    }

    fn new(inputs: &TestInputs, item: &ItemFn) -> Self {
        let definitions = inputs.definitions();
        let test_inputs = inputs.inputs();
        let inputs = &item.sig.inputs;
        let output = &item.sig.output;
        let test_body = &item.block;

        let body = quote! {
            fn __test_fn(#inputs) #output {
                #test_body
            }

            #definitions
            __test_fn(#test_inputs)
        };

        Self { body, output: item.sig.output.clone() }
    }
}

struct AsyncTest {
    body: TokenStream,
    output: ReturnType,
    terminator: Ident,
}

impl AsyncTest {
    fn body(&self) -> &TokenStream {
        &self.body
    }

    fn output(&self) -> &ReturnType {
        &self.output
    }

    fn new(inputs: &TestInputs, item: &ItemFn) -> Self {
        let definitions = inputs.definitions();
        let test_inputs = inputs.inputs();
        let inputs = &item.sig.inputs;
        let output = &item.sig.output;
        let test_body = &item.block;
        let terminator = Ident::new("__test_terminator", Span::call_site());

        let body = quote! {
            async fn __test_fn(#inputs) #output {
                #test_body
            }

            ::nomad::tests::async_body(#terminator, async move {
                #definitions
                __test_fn(#test_inputs).await
            })
        };

        Self { body, output: ReturnType::Default, terminator }
    }

    fn terminator(&self) -> FnArg {
        let terminator = &self.terminator;
        parse_quote! { #terminator: ::nomad::nvim::TestTerminator }
    }
}

#[derive(Parse)]
#[args(default)]
pub(super) struct Args {
    seed: SpecifiedSeed,
}

enum Seed {
    None,
    RandomlyGenerated,
    Specified(SpecifiedSeed),
}

impl Seed {
    /// Returns the `let seed = ...;` definition.
    fn definition(&self) -> proc_macro2::TokenStream {
        match self {
            Self::None => quote! {},

            Self::RandomlyGenerated => quote! {
                let seed = ::nomad::tests::random_seed();
            },

            Self::Specified(seed) => seed.definition(),
        }
    }

    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn name(&self) -> Ident {
        Ident::new("seed", Span::call_site())
    }

    fn new(
        args: &Args,
        inputs: &Punctuated<FnArg, Comma>,
    ) -> syn::Result<Self> {
        if args.seed.is_some() {
            return Ok(Self::Specified(args.seed.clone()));
        }

        let Some(first) = inputs.first() else {
            return Ok(Self::None);
        };

        let FnArg::Typed(pat) = first else {
            return Err(syn::Error::new_spanned(
                first,
                "expected a typed argument",
            ));
        };

        let Pat::Ident(pat_ident) = &*pat.pat else {
            return Err(syn::Error::new_spanned(
                pat,
                "expected an identifier",
            ));
        };

        let this = if pat_ident.ident == "gen" {
            Self::RandomlyGenerated
        } else {
            Self::None
        };

        Ok(this)
    }
}

#[derive(Clone, Default)]
enum SpecifiedSeed {
    Literal(LitInt),
    FromEnv,
    #[default]
    None,
}

impl SpecifiedSeed {
    /// Returns the `let seed = ...;` definition.
    fn definition(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Literal(seed) => {
                quote! {
                    let seed = #seed;
                }
            },

            Self::FromEnv => {
                quote! {
                    let seed = {
                        let Some(env) = ::std::env::var_os("SEED") else {
                            eprintln!("$SEED not set");
                            ::std::process::exit(1);
                        };
                        let Some(str) = env.to_str() else {
                            eprintln!("$SEED is not UTF-8");
                            ::std::process::exit(1);
                        };
                        match str.parse::<u64>() {
                            Ok(seed) => seed,
                            Err(err) => {
                                eprintln!("couldn't parse $SEED: {err}");
                                ::std::process::exit(1);
                            }
                        };
                    };
                }
            },

            Self::None => unreachable!(),
        }
    }

    fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

impl Parse for SpecifiedSeed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitInt) {
            let lit = input.parse()?;
            return Ok(Self::Literal(lit));
        }

        let _ = input.parse::<Dollar>()?;

        let seed = input.parse::<Ident>()?;

        if seed != "SEED" {
            return Err(syn::Error::new_spanned(
                seed,
                "expected `$SEED` or an integer",
            ));
        }

        Ok(Self::FromEnv)
    }
}
