use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote,
    token,
    Block,
    Error,
    Expr,
    ImplItem,
    ItemImpl,
    ReturnType,
    Signature,
    Stmt,
    Type,
    TypeImplTrait,
    TypeParamBound,
    TypeTuple,
};

pub fn async_action(mut item: ItemImpl) -> Result<TokenStream, Error> {
    check_return_is_unit(&item)?;

    let execute = item.items.iter_mut().find_map(|item| match item {
        ImplItem::Fn(func) if func.sig.ident == "execute" => Some(func),
        _ => None,
    });

    let Some(execute) = execute else {
        // There is no `execute` method in the impl block. This'll already
        // result in a compile error, so we don't have do nothing here.
        return Ok(quote!(#item));
    };

    if execute.sig.asyncness.is_none() {
        return Err(Error::new_spanned(&execute.sig, ExecuteNotAsyncError));
    }

    transform_signature(&mut execute.sig);

    transform_block(&mut execute.block);

    Ok(quote!(#item))
}

/// Checks that the `Return` associated type of the action is `()`, and emits a
/// compile error if it isn't.
fn check_return_is_unit(item: &ItemImpl) -> Result<(), Error> {
    let return_ty = item.items.iter().find_map(|item| match item {
        ImplItem::Type(ty) if ty.ident == "Return" => Some(ty),
        _ => None,
    });

    let Some(return_ty) = return_ty else {
        // There is no `Return` type in the impl block. This'll already
        // result in a compile error, so we don't have do nothing here.
        return Ok(());
    };

    if return_ty.ty == unit_type() {
        Ok(())
    } else {
        Err(Error::new_spanned(return_ty, ReturnNotUnitError))
    }
}

fn transform_signature(old: &mut Signature) {
    old.asyncness = None;
    transform_return_type(&mut old.output);
}

fn transform_block(old: &mut Block) {
    let expr: Expr = parse_quote! {
        ::nomad::maybe_future::MaybeFutureEnum::from(async move { #old })
    };

    let stmt = Stmt::Expr(expr, None);

    old.stmts = vec![stmt];
}

fn transform_return_type(old: &mut ReturnType) {
    let ty = loop {
        match old {
            ReturnType::Default => {
                *old = ReturnType::Type(
                    Default::default(),
                    Box::new(unit_type()),
                );
            },

            ReturnType::Type(_, ty) => break &mut **ty,
        }
    };

    let bound = TypeParamBound::Verbatim(
        quote!(::nomad::maybe_future::MaybeFuture<Output = #ty>),
    );

    let new_ty = Type::ImplTrait(TypeImplTrait {
        impl_token: token::Impl::default(),
        bounds: core::iter::once(bound).collect(),
    });

    *ty = new_ty;
}

fn unit_type() -> Type {
    Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    })
}

struct ExecuteNotAsyncError;

impl core::fmt::Display for ExecuteNotAsyncError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "the `execute` method must be async")
    }
}

struct ReturnNotUnitError;

impl core::fmt::Display for ReturnNotUnitError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "the `Return` type must be `()`")
    }
}
