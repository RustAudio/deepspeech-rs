extern crate proc_macro2;
extern crate syn;
use std::collections::HashSet;
use std::io::Read;
use std::path::PathBuf;
use syn::export::ToTokens;

fn main() {
    let in_path = PathBuf::from("src/lib.rs");
    let mut in_file = std::fs::File::open(&in_path).unwrap();
    let mut raw_contents = String::new();
    in_file.read_to_string(&mut raw_contents).unwrap();
    let mut tree = syn::parse_file(&raw_contents).unwrap();

    let types_with_calls: HashSet<_> = tree
        .items
        .iter_mut()
        .filter_map(|item| match item {
            syn::Item::Impl(m) => Some(m),
            _ => None,
        })
        // Use `filter_map` instead of `filter` because `should_add_library` also modifies the method bodies.
        .filter_map(|m| if should_add_library(m) { Some(m) } else { None })
        .filter_map(|m| match m.self_ty.as_ref() {
            syn::Type::Path(pt) => pt.path.get_ident().cloned(),
            _ => None,
        })
        .collect();

    let struct_declarations = tree
        .items
        .iter_mut()
        .filter_map(|item| match item {
            syn::Item::Struct(strbdy) => Some(strbdy),
            _ => None,
        })
        .filter(|strbdy| types_with_calls.contains(&strbdy.ident));

    for strbdy in struct_declarations {
        match &mut strbdy.fields {
            syn::Fields::Named(flds) => {
                for fld in flds.named.iter_mut() {
                    let typepath = match &mut fld.ty {
                        syn::Type::Path(pt) => pt,
                        syn::Type::Ptr(ptr) => match ptr.elem.as_mut() {
                            syn::Type::Path(pt) => pt,
                            _ => {
                                continue;
                            }
                        },
                        _ => {
                            continue;
                        }
                    };
                    let is_sys_call = typepath
                        .path
                        .segments
                        .first()
                        .map_or(false, |seg| seg.ident.to_string() == "ds");
                    if is_sys_call {
                        typepath.path.segments.insert(1, syn::parse_quote!(dynamic));
                    }
                }
                let vis: syn::Visibility = syn::parse_quote!();
                let colon_token: syn::token::Colon = syn::parse_quote!(:);
                let ty: syn::Type = syn::parse_quote!(std::sync::Arc<ds::dynamic::LibraryWrapper>);
                let field = syn::Field {
                    attrs: Vec::new(),
                    vis,
                    ident: Some(syn::parse_quote!(library)),
                    colon_token: Some(colon_token),
                    ty,
                };
                flds.named.insert(0, field);
            }
            syn::Fields::Unnamed(_) => todo!(),
            syn::Fields::Unit => todo!(),
        }
    }
    let out_path = in_path.with_file_name("dynamic.rs");
    eprintln!("Got library types: {:?}", types_with_calls);
    std::fs::write(out_path, format!("{}", tree.into_token_stream())).unwrap();
}

/// Checks whether this struct will need a reference to the dynamic library,
/// as well as converting static dispatch calls to one using the Arc<LibraryWrapper>.
pub fn should_add_library(m: &mut syn::ItemImpl) -> bool {
    m.items
        .iter_mut()
        .map(|itm| match itm {
            syn::ImplItem::Method(method) => visit_impl_method(method),
            _ => false,
        })
        .fold(false, |a, b| a | b)
}

/// Visits a struct method in the tree passed to `should_add_library`.
fn visit_impl_method(meth: &mut syn::ImplItemMethod) -> bool {
    meth.block
        .stmts
        .iter_mut()
        .map(|stm| visit_statement(stm))
        .fold(false, |a, b| a | b)
}

/// Visits a path in the tree passed to `should_add_library`.
///
/// This is a terminal node: if the path starts with "ds" it gets mapped
/// to "ds::dynamic::LibraryWrapper" and visit function returns `true`. Else, the function
/// does nothing and returns `false`.
fn visit_path(pt: &mut syn::ExprPath) -> bool {
    let is_sys_call = pt
        .path
        .segments
        .first()
        .map_or(false, |seg| seg.ident.to_string() == "ds");
    if is_sys_call {
        pt.path.segments.insert(1, syn::parse_quote!(dynamic));
        pt.path
            .segments
            .insert(2, syn::parse_quote!(LibraryWrapper));
        true
    } else {
        false
    }
}

/// Visits a function call in the tree passed to `should_add_library`.
///
/// If the call is found to invoke a "ds" system crate function, the
/// function call arguments are modified to start with a `&self.library` parameter.
fn visit_call(call: &mut syn::ExprCall) -> bool {
    match call.func.as_mut() {
        syn::Expr::Path(pt) => {
            if visit_path(pt) {
                let with_model: syn::Expr = syn::parse_quote!(&self.library);
                call.args.insert(0, with_model);
                true
            } else {
                false
            }
        }
        other => panic!("Invalid function invocation: {:?}", other),
    }
}

/// Visits a block in the tree passed to `should_add_library`.
fn visit_block(blk: &mut syn::Block) -> bool {
    let mut retvl = false;
    for item in blk.stmts.iter_mut() {
        retvl = retvl | visit_statement(item);
    }
    retvl
}

/// Visits an expression in the tree passed to `should_add_library`.
fn visit_expression(expr: &mut syn::Expr) -> bool {
    match expr {
        syn::Expr::Assign(inner) => {
            let lft_res = visit_expression(&mut inner.left);
            let rgt_res = visit_expression(&mut inner.right);
            lft_res | rgt_res
        }
        syn::Expr::AssignOp(inner) => {
            let lft_res = visit_expression(&mut inner.left);
            let rgt_res = visit_expression(&mut inner.right);
            lft_res | rgt_res
        }
        syn::Expr::Block(blk) => visit_block(&mut blk.block),
        syn::Expr::Call(cl) => {
            let res = visit_call(cl);
            if res {
                let cl = cl.to_token_stream();
                // Convert raw invocations into also `unwrap`ing the `libloading::Result`.
                let with_unwrap: syn::ExprMethodCall = syn::parse_quote!(#cl.unwrap());
                *expr = syn::Expr::MethodCall(with_unwrap);
                true
            } else {
                false
            }
        }
        syn::Expr::Cast(cst) => visit_expression(&mut cst.expr),
        syn::Expr::Closure(closure) => visit_expression(&mut closure.body),
        syn::Expr::Continue(_) => false,
        syn::Expr::Field(fld) => visit_expression(&mut fld.base),
        syn::Expr::ForLoop(frloop) => {
            let cond_res = visit_expression(&mut frloop.expr);
            let body_res = visit_block(&mut frloop.body);
            body_res | cond_res
        }
        syn::Expr::If(body) => {
            let then_res = visit_block(&mut body.then_branch);
            if let Some((_, else_branch)) = body.else_branch.as_mut() {
                let else_res = visit_expression(else_branch);
                then_res | else_res
            } else {
                then_res
            }
        }
        syn::Expr::Lit(_) => false,
        syn::Expr::Macro(_mcr) => {
            //TODO?
            false
        }
        syn::Expr::MethodCall(meth) => {
            let recv_res = visit_expression(&mut meth.receiver);
            let args_res = meth.args.iter_mut().map(|arg| visit_expression(arg));
            args_res.fold(recv_res, |a, b| a | b)
        }
        syn::Expr::Path(pt) => visit_path(pt),
        syn::Expr::Paren(inner) => visit_expression(&mut inner.expr),
        syn::Expr::Return(ret) => ret
            .expr
            .as_mut()
            .map_or(false, |expr| visit_expression(expr)),
        syn::Expr::Unary(unary) => visit_expression(&mut unary.expr),
        syn::Expr::Unsafe(body) => visit_block(&mut body.block),
        syn::Expr::While(whl) => {
            let cond_res = visit_expression(&mut whl.cond);
            let bdy_res = visit_block(&mut whl.body);
            cond_res | bdy_res
        }
        other => todo!("Unvisitable expression: {:?}", &other),
    }
}

/// Visits an item in the tree passed to `should_add_library`.
fn visit_item(itm: &mut syn::Item) -> bool {
    match itm {
        syn::Item::Struct(_) => false,
        syn::Item::TraitAlias(_) => false,
        syn::Item::Enum(_) => false,
        syn::Item::Static(_) => false,
        _ => todo!(),
    }
}

/// Visits a local variable declaration in the tree passed to `should_add_library`.
fn visit_local(lcl: &mut syn::Local) -> bool {
    if let Some((_, next)) = lcl.init.as_mut() {
        visit_expression(next)
    } else {
        false
    }
}

/// Visits a statement in the tree passed to `should_add_library`.
fn visit_statement(stmt: &mut syn::Stmt) -> bool {
    match stmt {
        syn::Stmt::Expr(ex) | syn::Stmt::Semi(ex, _) => visit_expression(ex),
        syn::Stmt::Item(itm) => visit_item(itm),
        syn::Stmt::Local(lcl) => visit_local(lcl),
    }
}
