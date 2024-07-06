use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{
    BlockStmt, CallExpr, Callee, Expr, ExprStmt, Lit, MemberExpr, MemberProp, Stmt, Str,
};
use swc_core::ecma::{
    ast::{Ident, Program},
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith as _},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl TransformVisitor {
    fn is_regex_identifier(e: &Expr) -> bool {
        match e {
            Expr::Ident(ident) => {
                // println!("ident.sym: {:?}", ident.sym);
                ident.sym == *"regex" || ident.sym == *"str"
            }
            Expr::Member(MemberExpr {
                obj,
                prop: MemberProp::Ident(_prop),
                ..
            }) => {
                // println!("obj===>: {:?}", obj);
                if let Expr::Ident(ident) = &**obj {
                    return ident.sym == *"regex" || ident.sym == *"str";
                }
                false
            }
            _ => false,
        }
    }
    fn is_regex_method(e: &Expr) -> bool {
        match e {
            Expr::Member(MemberExpr {
                obj: _,
                prop: MemberProp::Ident(prop),
                ..
            }) => {
                prop.sym == *"exec"
                    || prop.sym == *"match"
                    || prop.sym == *"test"
                    || prop.sym == *"replace"
            }
            _ => false,
        }
    }

    fn is_valid_identifier(e: &Expr) -> bool {
        // check if the identifier is a regex call match, exec, test
        match e {
            Expr::Member(member_expr) => {
                Self::is_regex_identifier(&member_expr.obj) && Self::is_regex_method(e)
            }
            _ => false,
        }
    }
}

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
    // fn visit_mut_bin_expr(&mut self, e: &mut BinExpr) {
    //     e.visit_mut_children_with(self);

    fn visit_mut_stmt(&mut self, n: &mut Stmt) {
        n.visit_mut_children_with(self);

        // use performance.now() to measure time
        if let Stmt::Expr(expr_stmt) = n {
            if let Expr::Call(call) = &*expr_stmt.expr {
                if let Callee::Expr(callee) = &call.callee {
                    if Self::is_valid_identifier(callee) {
                        let start_time_expr = Expr::Call(CallExpr {
                            span: DUMMY_SP,
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: Box::new(Expr::Ident(Ident::new("console".into(), DUMMY_SP))),
                                prop: MemberProp::Ident(Ident::new("time".into(), DUMMY_SP)),
                            }))),
                            args: vec![Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: "regex".into(),
                                raw: None,
                            }))
                            .into()],
                            type_args: None,
                        });

                        let end_time_expr = Expr::Call(CallExpr {
                            span: DUMMY_SP,
                            callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: Box::new(Expr::Ident(Ident::new("console".into(), DUMMY_SP))),
                                prop: MemberProp::Ident(Ident::new("timeEnd".into(), DUMMY_SP)),
                            }))),
                            args: vec![Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: "regex".into(),
                                raw: None,
                            }))
                            .into()],
                            type_args: None,
                        });

                        let new_stmt = Stmt::Block(BlockStmt {
                            span: DUMMY_SP,
                            stmts: vec![
                                Stmt::Expr(ExprStmt {
                                    span: DUMMY_SP,
                                    expr: Box::new(start_time_expr),
                                }),
                                Stmt::Expr(ExprStmt {
                                    span: DUMMY_SP,
                                    expr: Box::new(Expr::Call(CallExpr {
                                        span: DUMMY_SP,
                                        callee: call.callee.clone(),
                                        args: call.args.clone(),
                                        type_args: call.type_args.clone(),
                                    })),
                                }),
                                Stmt::Expr(ExprStmt {
                                    span: DUMMY_SP,
                                    expr: Box::new(end_time_expr),
                                }),
                            ],
                        });

                        *n = new_stmt;
                    }
                }
            }
        }
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
